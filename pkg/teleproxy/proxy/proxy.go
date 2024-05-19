package proxy

import (
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"strings"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	"beleap.dev/teleproxy/pkg/teleproxy/util"
	"go.uber.org/zap"
)

var (
	_ http.Handler = &proxyHandler{}
)

// Hop-by-hop headers. These are removed when sent to the backend.
// http://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html
var hopHeaders = []string{
	"Connection",
	"Keep-Alive",
	"Proxy-Authenticate",
	"Proxy-Authorization",
	"Te", // canonicalized version of "TE"
	"Trailers",
	"Transfer-Encoding",
	"Upgrade",
}

func copyHeader(dst, src http.Header) {
	for k, vv := range src {
		for _, v := range vv {
			dst.Add(k, v)
		}
	}
}

func delHopHeaders(header http.Header) {
	for _, h := range hopHeaders {
		header.Del(h)
	}
}

func appendHostToXForwardHeader(header http.Header, host string) {
	// If we aren't the first proxy retain prior
	// X-Forwarded-For information as a comma+space
	// separated list and fold multiple headers into one.
	if prior, ok := header["X-Forwarded-For"]; ok {
		host = strings.Join(prior, ", ") + ", " + host
	}
	header.Set("X-Forwarded-For", host)
}

type proxyHandler struct {
	target     *url.URL
	spyconfigs *spyconfigs.SpyConfigs

	requestChan  map[string]chan *httprequest.HttpRequestDto
	responseChan chan *httpresponse.HttpResponseDto
}

func (p *proxyHandler) ServeHTTP(wr http.ResponseWriter, req *http.Request) {
	util.GetLogger().Debug(req.RemoteAddr + " " + req.Method + " " + req.URL.String())

	req.RequestURI = ""
	delHopHeaders(req.Header)
	if clientIP, _, err := net.SplitHostPort(req.RemoteAddr); err == nil {
		appendHostToXForwardHeader(req.Header, clientIP)
	}

	matching, err := p.spyconfigs.GetMatching(req.Header)
	if err != nil && !errors.Is(err, spyconfigs.NoMatchingError) {
		util.GetLogger().Error("Exception on get matching", zap.Error(err))
	}

	var resp *http.Response
	if !errors.Is(err, spyconfigs.NoMatchingError) {
		util.GetLogger().Debug("Match result: " + matching)
		httpRequest, err := httprequest.FromHttpRequest(req)
		if err != nil {
			http.Error(wr, "Server Error", http.StatusInternalServerError)
			util.GetLogger().Error("Failed to proxy spied request", zap.Error(err))
			return
		}

		p.requestChan[matching] <- httpRequest
		respDto := <-p.responseChan
		resp = respDto.ToHttpResponse()
		util.GetLogger().Debug("Resp as " + resp.Status)
	} else {
		client := &http.Client{}
		req.URL.Scheme = p.target.Scheme
		req.URL.Host = p.target.Host
		resp, err = client.Do(req)
		if err != nil {
			http.Error(wr, "Server Error", http.StatusInternalServerError)
			util.GetLogger().Error("Failed to proxy request", zap.Error(err))
			return
		}
	}
	defer resp.Body.Close()

	delHopHeaders(resp.Header)

	copyHeader(wr.Header(), resp.Header)
	wr.WriteHeader(resp.StatusCode)
	io.Copy(wr, resp.Body)
}

func Start(requestChan map[string]chan *httprequest.HttpRequestDto, responseChan chan *httpresponse.HttpResponseDto, configs *spyconfigs.SpyConfigs, port int, targetRaw string) {
	util.GetLogger().Info("Proxying request to: " + targetRaw)
	target, err := url.Parse(targetRaw)
	if err != nil {
		util.GetLogger().Error("Failed to parse target", zap.Error(err))
	}

	s := &http.Server{
		Addr: fmt.Sprintf(":%d", port),
		Handler: &proxyHandler{
			target:     target,
			spyconfigs: configs,

			requestChan:  requestChan,
			responseChan: responseChan,
		},
	}
	util.GetLogger().Info("Listening on " + s.Addr)
	util.GetLogger().Error("", zap.Error(s.ListenAndServe()))
}

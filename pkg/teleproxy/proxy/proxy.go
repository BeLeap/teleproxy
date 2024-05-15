package proxy

import (
	"errors"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"net/url"
	"os"
	"strings"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
)

var (
	_      http.Handler = &proxyHandler{}
	logger              = log.New(os.Stdout, "[proxy] ", log.LstdFlags|log.Lmicroseconds)
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

	idChan       chan string
	requestChan  chan *httprequest.HttpRequestDto
	responseChan chan *httpresponse.HttpResponseDto
}

func (p *proxyHandler) ServeHTTP(wr http.ResponseWriter, req *http.Request) {
  logger.Printf("%s %s", req.Method, req.URL)

	matching, err := p.spyconfigs.GetMatching(req.Header)
	if err != nil && !errors.Is(err, spyconfigs.NoMatchingError) {
		logger.Printf("Exception on get matching: %v", err)
	}

	req.RequestURI = ""
	delHopHeaders(req.Header)
	if clientIP, _, err := net.SplitHostPort(req.RemoteAddr); err == nil {
		appendHostToXForwardHeader(req.Header, clientIP)
	}

	var resp *http.Response
	if !errors.Is(err, spyconfigs.NoMatchingError) {
		logger.Printf("Match result: %v", matching)

		p.idChan <- matching
		httpRequest, err := httprequest.FromHttpRequest(req)
		if err != nil {
			http.Error(wr, "Server Error", http.StatusInternalServerError)
			logger.Print("Failed to proxy request: ", err)
			return
		}

		p.requestChan <- httpRequest
		respDto := <-p.responseChan
		resp = respDto.ToHttpResponse()
		logger.Printf("Resp: %v", resp)
	} else {
		client := &http.Client{}
		req.URL = p.target
		resp, err = client.Do(req)
		if err != nil {
			http.Error(wr, "Server Error", http.StatusInternalServerError)
			logger.Print("Failed to proxy request: ", err)
			return
		}
	}
	defer resp.Body.Close()

	delHopHeaders(resp.Header)

	copyHeader(wr.Header(), resp.Header)
	wr.WriteHeader(resp.StatusCode)
	io.Copy(wr, resp.Body)
}

func Start(idChan chan string, requestChan chan *httprequest.HttpRequestDto, responseChan chan *httpresponse.HttpResponseDto, configs *spyconfigs.SpyConfigs, port int, targetRaw string) {
  logger.Printf("Proxying request to: %s", targetRaw)
	target, err := url.Parse(targetRaw)
	if err != nil {
		logger.Fatalf("Failed to parse target: %v", err)
	}

	s := &http.Server{
		Addr: fmt.Sprintf(":%d", port),
		Handler: &proxyHandler{
			target:     target,
			spyconfigs: configs,

			idChan:       idChan,
			requestChan:  requestChan,
			responseChan: responseChan,
		},
	}
	logger.Printf("Listening on %s", s.Addr)
	logger.Println(s.ListenAndServe())
}

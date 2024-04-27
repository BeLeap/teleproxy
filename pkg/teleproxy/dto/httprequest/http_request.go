package httprequest

import (
	"bytes"
	"io"
	"net/http"
	"net/url"

	headervalues "beleap.dev/teleproxy/pkg/teleproxy/dto/header_values"
	pb "beleap.dev/teleproxy/protobuf"
)

type HttpRequestDto struct {
	Method string
	Url    *url.URL
	Header map[string][]string
	Body   []byte
}

func FromHttpRequest(req *http.Request) (*HttpRequestDto, error) {
	b, err := io.ReadAll(req.Body)
	if err != nil {
		return nil, err
	}
	return &HttpRequestDto{
		Method: req.Method,
		Url:    req.URL,
		Header: req.Header,
		Body:   b,
	}, nil
}

func (r *HttpRequestDto) ToPb() *pb.ListenResponse {
	return &pb.ListenResponse{
		Method: r.Method,
		Url:    r.Url.String(),
		Header: headervalues.ToPb(r.Header),
		Body:   r.Body,
	}
}

func FromPb(in *pb.ListenResponse) (*HttpRequestDto, error) {
	url, err := url.Parse(in.Url)
	if err != nil {
		return nil, err
	}

	return &HttpRequestDto{
		Method: in.Method,
		Url:    url,
		Header: headervalues.FromPb(in.Header),
		Body:   in.Body,
	}, nil
}

func (r *HttpRequestDto) ToHttpRequest() (*http.Request, error) {
	req, err := http.NewRequest(r.Method, r.Url.String(), bytes.NewReader(r.Body))
	if err != nil {
		return nil, err
	}
	req.Header = r.Header

	return req, nil
}

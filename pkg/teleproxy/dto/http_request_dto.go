package http_request_dto

import (
	"bytes"
	"io"
	"net/http"
	"net/url"

	pb "beleap.dev/teleproxy/protobuf"
)

type HttpRequestDto struct {
	Method  string
	Url     *url.URL
	Headers map[string][]string
	Body    []byte
}

func FromHttpRequest(req *http.Request) (*HttpRequestDto, error) {
	b, err := io.ReadAll(req.Body)
	if err != nil {
		return nil, err
	}
	return &HttpRequestDto{
		Method:  req.Method,
		Url:     req.URL,
		Headers: req.Header,
		Body:    b,
	}, nil
}

func (r *HttpRequestDto) ToPb() pb.ListenResponse {
	headers := map[string]*pb.HeaderValues{}
	for k, v := range r.Headers {
		headers[k] = &pb.HeaderValues{
			Values: v,
		}
	}

	return pb.ListenResponse{
		Method:  r.Method,
		Url:     r.Url.String(),
		Headers: headers,
		Body:    r.Body,
	}
}

func FromPb(in *pb.ListenResponse) (*HttpRequestDto, error) {
	url, err := url.Parse(in.Url)
	if err != nil {
		return nil, err
	}

	headers := map[string][]string{}
	for k, v := range in.Headers {
		headers[k] = v.Values
	}

	return &HttpRequestDto{
		Method:  in.Method,
		Url:     url,
		Headers: headers,
		Body:    in.Body,
	}, nil
}

func (r *HttpRequestDto) ToHttpRequest() (*http.Request, error) {
	req, err := http.NewRequest(r.Method, r.Url.String(), bytes.NewReader(r.Body))
	if err != nil {
		return nil, err
	}
	req.Header = r.Headers

	return req, nil
}

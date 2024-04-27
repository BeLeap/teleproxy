package http_response

import (
	"io"
	"net/http"
)

type HttpResponseDto struct {
	Status string
	Proto  string
	Header map[string][]string
	Body   []byte
}

func FromHttpResponse(in *http.Response) (*HttpResponseDto, error) {
	body, err := io.ReadAll(in.Body)
	if err != nil {
		return nil, err
	}

	return &HttpResponseDto{
		Status: in.Status,
		Proto:  in.Proto,
		Header: in.Header,
		Body:   body,
	}, nil
}

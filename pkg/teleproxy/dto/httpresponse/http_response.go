package httpresponse

import (
	"io"
	"net/http"

	headervalues "beleap.dev/teleproxy/pkg/teleproxy/dto/header_values"
	pb "beleap.dev/teleproxy/protobuf"
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

func (d *HttpResponseDto) ToPb(apiKey string, id string) *pb.ListenRequest {
	return &pb.ListenRequest{
		ApiKey: apiKey,
		Id: id,

		Status: d.Status,
		Proto: d.Proto,
		Header: headervalues.ToPb(d.Header),
		Body: d.Body,
	}
}

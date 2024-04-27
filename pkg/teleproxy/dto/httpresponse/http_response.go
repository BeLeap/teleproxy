package httpresponse

import (
	"bytes"
	"io"
	"net/http"

	headervalues "beleap.dev/teleproxy/pkg/teleproxy/dto/header_values"
	pb "beleap.dev/teleproxy/protobuf"
)

type HttpResponseDto struct {
	Status     string
	StatusCode int
	Proto      string
	ProtoMajor int
	ProtoMinor int
	Header     map[string][]string
	Body       []byte
}

var InternalServerError = &HttpResponseDto{
	Status: "500 Internal Server Error",
	StatusCode: 500,
	Proto: "HTTP/1.1",
	ProtoMajor: 1,
	ProtoMinor: 1,
}

func FromPb(in *pb.ListenRequest) *HttpResponseDto {
	return &HttpResponseDto{
		Status: in.Status,
		StatusCode: int(in.StatusCode),
		Proto:  in.Proto,
		ProtoMajor: int(in.ProtoMajor),
		ProtoMinor: int(in.ProtoMinor),
		Header: headervalues.FromPb(in.Header),
		Body:   in.Body,
	}
}

func FromHttpResponse(in *http.Response) (*HttpResponseDto, error) {
	body, err := io.ReadAll(in.Body)
	if err != nil {
		return nil, err
	}

	return &HttpResponseDto{
		Status:     in.Status,
		StatusCode: in.StatusCode,
		Proto:      in.Proto,
		ProtoMajor: in.ProtoMajor,
		ProtoMinor: in.ProtoMinor,
		Header:     in.Header,
		Body:       body,
	}, nil
}

func (d *HttpResponseDto) ToPb(apiKey string, id string) *pb.ListenRequest {
	return &pb.ListenRequest{
		ApiKey: apiKey,
		Id:     id,

		Status:     d.Status,
		StatusCode: int32(d.StatusCode),
		Proto:      d.Proto,
		ProtoMajor: int32(d.ProtoMajor),
		ProtoMinor: int32(d.ProtoMinor),
		Header:     headervalues.ToPb(d.Header),
		Body:       d.Body,
	}
}

func (d *HttpResponseDto) ToHttpResponse() *http.Response {
	return &http.Response{
		Status:     d.Status,
		StatusCode: d.StatusCode,
		Proto:      d.Proto,
		ProtoMajor: d.ProtoMajor,
		ProtoMinor: d.ProtoMinor,
		Header:     d.Header,
		Body:       io.NopCloser(bytes.NewReader(d.Body)),
	}
}

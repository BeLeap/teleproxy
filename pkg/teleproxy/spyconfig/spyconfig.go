package spyconfig

import (
	"fmt"

	"github.com/oklog/ulid/v2"
)

type SpyConfig struct {
	Id string

	HeaderKey   string
	HeaderValue string

	To string
}

func New(headerKey string, headerValue string, to string) SpyConfig {
	return SpyConfig {
		Id: fmt.Sprint(ulid.Make()),

		HeaderKey: headerKey,
		HeaderValue: headerValue,

		To: to,
	}
}

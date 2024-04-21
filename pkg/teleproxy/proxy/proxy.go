package proxy

import (
	"fmt"
	"log"
	"net/http"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
)

var (
	_      http.Handler = &proxyHandler{}
	logger              = log.New(os.Stdout, "[proxy] ", log.LstdFlags|log.Lmicroseconds)
)

type proxyHandler struct{}

func (p *proxyHandler) ServeHTTP(http.ResponseWriter, *http.Request) {
	logger.Print("Request Recv")
}

func StartProxy(configs *spyconfigs.SpyConfigs, port int) {
	s := &http.Server{
		Addr:    fmt.Sprintf(":%d", port),
		Handler: &proxyHandler{},
	}
	logger.Fatal(s.ListenAndServe())
}

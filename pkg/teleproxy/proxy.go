package teleproxy

import (
	"fmt"
	"log"
	"net/http"
)

var (
	_ http.Handler = &proxyHandler{}
)

type proxyHandler struct{}

func (p *proxyHandler) ServeHTTP(http.ResponseWriter, *http.Request) {
	log.Print("request recv")
}

func StartProxy(port int) {
	s := &http.Server{
		Addr: fmt.Sprintf(":%d", port),
		Handler: &proxyHandler {},
	}
	log.Fatal(s.ListenAndServe())
}

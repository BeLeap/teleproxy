package teleproxy

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

var (
	_ http.Handler = &proxyHandler{}
	logger = log.New(os.Stdout, "[proxy] ", log.LstdFlags | log.Lmicroseconds)
)

type proxyHandler struct{}

func (p *proxyHandler) ServeHTTP(http.ResponseWriter, *http.Request) {
	logger.Print("Request Recv")
}

func StartProxy(port int) {
	s := &http.Server{
		Addr: fmt.Sprintf(":%d", port),
		Handler: &proxyHandler {},
	}
	logger.Fatal(s.ListenAndServe())
}

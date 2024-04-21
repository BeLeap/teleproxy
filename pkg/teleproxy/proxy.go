package teleproxy

import (
	"fmt"
	"log"
	"net/http"
)

func StartProxy(port int) {
	s := &http.Server {
		Addr: fmt.Sprintf(":%d", port),
	}
	log.Fatal(s.ListenAndServe())
}

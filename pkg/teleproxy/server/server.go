package server

import (
	"fmt"
	"log"
	"net/http"
)

func StartServer(port int) {
	log.Fatal(http.ListenAndServe(
		fmt.Sprintf(":%d", port),
		nil,
	))
}

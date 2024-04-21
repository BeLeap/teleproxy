package server

import (
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
)

func StartServer(port int) {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}

	grpcServer := grpc.NewServer()
	grpcServer.Serve(lis)
}

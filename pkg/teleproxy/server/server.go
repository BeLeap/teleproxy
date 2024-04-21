package server

import (
	"fmt"
	"log"
	"net"
	"time"

	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
)

type teleProxyServer struct {
	pb.UnimplementedTeleProxyServer
}

func (s *teleProxyServer) Listen(request *pb.ListenRequest, stream pb.TeleProxy_ListenServer) error {
	log.Println("[server] recv")
	for true {
		time.Sleep(1000)
		err := stream.Send(&pb.Http{
			Method: "GET",
		})
		if err != nil {
			log.Fatalf("[server] failed to send response: %v", err)
			return err
		}
	}
	return nil
}

func StartServer(port int) {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterTeleProxyServer(grpcServer, &teleProxyServer{})
	grpcServer.Serve(lis)
}

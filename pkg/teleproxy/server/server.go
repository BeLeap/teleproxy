package server

import (
	"fmt"
	"io"
	"log"
	"net"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
)

var logger = log.New(os.Stdout, "[server] ", log.LstdFlags|log.Lmicroseconds)

type teleProxyServer struct {
	pb.UnimplementedTeleProxyServer
	configs *spyconfigs.SpyConfigs
}

func (s *teleProxyServer) Listen(request *pb.ListenRequest, stream pb.TeleProxy_ListenServer) error {
	logger.Println("Recv")
	config := spyconfig.New(request.HeaderKey, request.HeaderValue)
	s.configs.AddSpyConfig(config)

	for {
		err := stream.Send(&pb.Http{
			Method: "GET",
		})
		if err == io.EOF {
			logger.Print("Client closed connection")
			break
		}
		if err != nil {
			logger.Printf("Failed to send response: %v", err)
			break
		}
	}
	return nil
}

func StartServer(configs *spyconfigs.SpyConfigs, port int) {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		logger.Fatalf("Failed to start server: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterTeleProxyServer(grpcServer, &teleProxyServer{
		configs: configs,
	})
	logger.Printf("Listening on %s", lis.Addr().String())
	grpcServer.Serve(lis)
}

package server

import (
	"context"
	"fmt"
	"io"
	"log"
	"net"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

var (
	logger = log.New(os.Stdout, "[server] ", log.LstdFlags|log.Lmicroseconds)
	apiKey = os.Getenv("API_KEY")
)

type teleProxyServer struct {
	pb.UnimplementedTeleProxyServer
	configs *spyconfigs.SpyConfigs
}

func (s *teleProxyServer) Listen(req *pb.ListenRequest, stream pb.TeleProxy_ListenServer) error {
	if req.ApiKey != apiKey {
		logger.Print("Not matching api key")
		return status.Error(codes.Unauthenticated, "Not matching api key")
	}

	logger.Println("Recv")
	config := spyconfig.New(req.HeaderKey, req.HeaderValue)
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

	s.configs.RemoveSpyConfig(config.Id)
	return nil
}
func (s *teleProxyServer) Dump(ctx context.Context, req *pb.DumpRequest) (*pb.DumpResponse, error) {
	if req.ApiKey != apiKey {
		logger.Print("Not matching api key")
		return nil, status.Error(codes.Unauthenticated, "Not matching api key")
	}

	res, err := s.configs.DumpSpyConfigs()
	if err != nil {
		logger.Printf("Failed to dump spy configs: %v", err)
		return nil, status.Error(codes.Internal, "Failed to dump spy configs")
	}
	resp := &pb.DumpResponse{
		Dump: res,
	}
	return resp, nil
}

func Start(configs *spyconfigs.SpyConfigs, port int) {
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

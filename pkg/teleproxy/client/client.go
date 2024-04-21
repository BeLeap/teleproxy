package client

import (
	"context"
	"io"
	"log"
	"os"

	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func StartListen(serverAddr string) {
	opts := []grpc.DialOption{
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		log.Fatalf("Failed to dial grpc server: %v", err)
		os.Exit(1)
	}
	defer conn.Close()

	client := pb.NewTeleProxyClient(conn)

	stream, err := client.Listen(context.Background(), &pb.Empty{})
	if err != nil {
		log.Fatalf("Failed to call client.Listen: %v", err)
		os.Exit(1)
	}
	for true {
		http, err := stream.Recv()
		if err == io.EOF {
			return
		}
		if err != nil {
			log.Fatalf("failed to listen: %v", err)
		}
		log.Printf("recv: %v", http)
	}
}

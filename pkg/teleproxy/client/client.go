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

var logger = log.New(os.Stdout, "[client] ", log.LstdFlags|log.Lmicroseconds)

func StartListen(serverAddr string, key string, value string) {
	opts := []grpc.DialOption{
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		logger.Fatalf("Failed to dial grpc server: %v", err)
		os.Exit(1)
	}
	defer conn.Close()

	client := pb.NewTeleProxyClient(conn)

	stream, err := client.Listen(context.Background(), &pb.ListenRequest{
		HeaderKey:   key,
		HeaderValue: value,
	})
	if err != nil {
		logger.Fatalf("Failed to call client.Listen: %v", err)
		os.Exit(1)
	}

	for {
		http, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			logger.Fatalf("Failed to listen: %v", err)
		}
		logger.Printf("Recv: %v", http)
	}
}

func Dump(serverAddr string) {
	opts := []grpc.DialOption{
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		logger.Fatalf("Failed to dial grpc server: %v", err)
		os.Exit(1)
	}
	defer conn.Close()

	client := pb.NewTeleProxyClient(conn)

	resp, err := client.Dump(context.Background(), &pb.DumpRequest{})
	if err != nil {
		logger.Fatalf("Failed to call client.Dump: %v", err)
	}

	logger.Print(resp)
}

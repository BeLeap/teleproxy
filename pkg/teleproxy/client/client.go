package client

import (
	"context"
	"io"
	"log"
	"os"
	"sync"

	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var logger = log.New(os.Stdout, "[client] ", log.LstdFlags|log.Lmicroseconds)

func StartListen(ctx context.Context, wg *sync.WaitGroup, serverAddr string, apikey string, key string, value string) {
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

	config, err := client.Register(ctx, &pb.RegisterRequest{
		ApiKey:      apikey,
		HeaderKey:   key,
		HeaderValue: value,
	})
	if err != nil {
		logger.Fatalf("Failed to call client.Register: %v", err)
	}
	logger.Printf("Registered with Id: %s", config.Id)

	stream, err := client.Listen(ctx)
	if err != nil {
		logger.Fatalf("Failed to call client.Listen: %v", err)
		os.Exit(1)
	}
	stream.Send(&pb.ListenRequest{
		ApiKey: apikey,
		Id:     config.Id,
	})

	for {
		select {
		case <-ctx.Done():
			client.Deregister(context.Background(), &pb.DeregisterRequest{
				ApiKey: apikey,
				Id:     config.Id,
			})
			logger.Printf("Deregistered with Id: %s", config.Id)
			wg.Done()
			return
		default:
			http, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				logger.Printf("Failed to listen: %v", err)
			}
			logger.Printf("Recv: %v", http)
			stream.Send(&pb.ListenRequest{
				ApiKey: apikey,
				Id:     config.Id,
			})
		}
	}
}

func Dump(serverAddr string, apikey string) {
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

	resp, err := client.Dump(context.Background(), &pb.DumpRequest{
		ApiKey: apikey,
	})
	if err != nil {
		logger.Fatalf("Failed to call client.Dump: %v", err)
	}

	logger.Print(resp)
}

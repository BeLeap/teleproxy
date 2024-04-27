package client

import (
	"context"
	"log"
	"os"

	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var logger = log.New(os.Stdout, "[client] ", log.LstdFlags|log.Lmicroseconds)

func StartListen(serverAddr string, apikey string, key string, value string) {
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

	config, err := client.Register(context.Background(), &pb.RegisterRequest{
		ApiKey:      apikey,
		HeaderKey:   key,
		HeaderValue: value,
	})
	if err != nil {
		logger.Fatalf("Failed to call client.Register: %v", err)
	}
	logger.Printf("Registered with Id: %s", config.Id)
	defer client.Deregister(context.Background(), &pb.DeregisterRequest{
		ApiKey: apikey,
		Id:     config.Id,
	})

	// stream, err := client.Listen(context.Background(), &pb.ListenRequest{ ApiKey: apikey, HeaderKey: "dummy", HeaderValue: "dummy" })
	// if err != nil {
	// 	logger.Fatalf("Failed to call client.Listen: %v", err)
	// 	os.Exit(1)
	// }
	//
	// for {
	// 	http, err := stream.Recv()
	// 	if err == io.EOF {
	// 		break
	// 	}
	// 	if err != nil {
	// 		logger.Fatalf("Failed to listen: %v", err)
	// 	}
	// 	logger.Printf("Recv: %v", http)
	// }
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

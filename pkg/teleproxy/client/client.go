package client

import (
	"context"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
	pb "beleap.dev/teleproxy/protobuf"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var logger = log.New(os.Stdout, "[client] ", log.LstdFlags|log.Lmicroseconds)

func StartListen(ctx context.Context, wg *sync.WaitGroup, serverAddr string, apikey string, key string, value string, target string) {
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

	httpClient := &http.Client{}

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
			listenResp, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				logger.Printf("Failed to listen: %v", err)
			}

			httpRequestDto, err := httprequest.FromPb(listenResp)
			if err != nil {
				stream.Send(&pb.ListenRequest{
					ApiKey: apikey,
					Id:     config.Id,
				})
				stream.CloseSend()
				logger.Fatalf("Failed convert to dto: %v", err)
			}
			httpReq, err := httpRequestDto.ToHttpRequest()
			if err != nil {
				stream.Send(&pb.ListenRequest{
					ApiKey: apikey,
					Id:     config.Id,
				})
				stream.CloseSend()
				logger.Fatalf("Failed to create request: %v", err)
			}

			httpReq.URL, err = url.Parse(target)
			if err != nil {
				stream.Send(&pb.ListenRequest{
					ApiKey: apikey,
					Id:     config.Id,
				})
				stream.CloseSend()
				logger.Fatalf("Failed to parse target info: %v", err)
			}

			resp, err := httpClient.Do(httpReq)
			if err != nil {
				stream.Send(&pb.ListenRequest{
					ApiKey: apikey,
					Id:     config.Id,
				})
				stream.CloseSend()
				logger.Fatalf("Failed to handle request: %v", err)
			}

			logger.Printf("Resp: %v", resp)
			httpResponse, err := httpresponse.FromHttpResponse(resp)
			if err != nil {
				stream.Send(&pb.ListenRequest{
					ApiKey: apikey,
					Id:     config.Id,
				})
				stream.CloseSend()
				logger.Fatalf("Failed to handle request: %v", err)
			}

			stream.Send(httpResponse.ToPb(apikey, config.Id))
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

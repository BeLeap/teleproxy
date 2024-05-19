package client

import (
	"context"
	"crypto/tls"
	"io"
	"net/http"
	"net/url"
	"os"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
	"beleap.dev/teleproxy/pkg/teleproxy/util"
	pb "beleap.dev/teleproxy/protobuf"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
)

func StartListen(ctx context.Context, wg *sync.WaitGroup, serverAddr string, apikey string, key string, value string, target string, useInsecure bool) {
	var opts []grpc.DialOption
	if useInsecure {
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(insecure.NewCredentials()),
		}
	} else {
		creds := credentials.NewTLS(&tls.Config{})
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(creds),
		}
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		util.GetLogger().Error("Failed to dial grpc server", zap.Error(err))
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
		util.GetLogger().Error("Failed to call client.Register", zap.Error(err))
		os.Exit(1)
	}
	util.GetLogger().Info("Registered with Id: " + config.Id)
	wg.Add(1)
	defer func() {
		client.Deregister(context.Background(), &pb.DeregisterRequest{
			ApiKey: apikey,
			Id:     config.Id,
		})
		util.GetLogger().Info("Deregistered with Id: " + config.Id)
		wg.Done()
	}()

	stream, err := client.Listen(ctx)
	if err != nil {
		util.GetLogger().Error("Failed to call client.Listen", zap.Error(err))
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
			return
		default:
			listenResp, err := stream.Recv()
			if err == io.EOF {
				break
			}
			if err != nil {
				util.GetLogger().Error("Failed to listen", zap.Error(err))
				return
			}

			httpRequestDto, err := httprequest.FromPb(listenResp)
			if err != nil {
				stream.Send(httpresponse.InternalServerError.ToPb(apikey, config.Id))
				stream.CloseSend()
				util.GetLogger().Error("Failed convert to dto", zap.Error(err))
				break
			}
			util.GetLogger().Info("Handling request: " + httpRequestDto.Method + " " + httpRequestDto.Url.String())

			httpReq, err := httpRequestDto.ToHttpRequest()
			if err != nil {
				stream.Send(httpresponse.InternalServerError.ToPb(apikey, config.Id))
				stream.CloseSend()
				util.GetLogger().Error("Failed to create request", zap.Error(err))
				break
			}

			util.GetLogger().Info(httpReq.RemoteAddr + " " + httpReq.Method + " " + httpReq.URL.String())
			targetURL, err := url.Parse(target)
			httpReq.URL.Scheme = targetURL.Scheme
			httpReq.URL.Host = targetURL.Host
			if err != nil {
				stream.Send(httpresponse.InternalServerError.ToPb(apikey, config.Id))
				stream.CloseSend()
				util.GetLogger().Error("Failed to parse target info", zap.Error(err))
				break
			}

			resp, err := httpClient.Do(httpReq)
			if err != nil {
				stream.Send(httpresponse.InternalServerError.ToPb(apikey, config.Id))
				stream.CloseSend()
				util.GetLogger().Error("Failed to handle request", zap.Error(err))
				break
			}

			httpResponse, err := httpresponse.FromHttpResponse(resp)
			if err != nil {
				stream.Send(httpresponse.InternalServerError.ToPb(apikey, config.Id))
				stream.CloseSend()
				util.GetLogger().Error("Failed to handle request", zap.Error(err))
				break
			}

			stream.Send(httpResponse.ToPb(apikey, config.Id))
		}
	}
}

func Dump(serverAddr string, apikey string, useInsecure bool) {
	var opts []grpc.DialOption
	if useInsecure {
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(insecure.NewCredentials()),
		}
	} else {
		creds := credentials.NewTLS(&tls.Config{})
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(creds),
		}
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		util.GetLogger().Error("Failed to dial grpc server", zap.Error(err))
		os.Exit(1)
	}
	defer conn.Close()

	client := pb.NewTeleProxyClient(conn)

	resp, err := client.Dump(context.Background(), &pb.DumpRequest{
		ApiKey: apikey,
	})
	if err != nil {
		util.GetLogger().Error("Failed to call client.Dump", zap.Error(err))
		os.Exit(1)
	}

	util.GetLogger().Info(resp.String())
}

func Flush(serverAddr string, apikey string, useInsecure bool) {
	var opts []grpc.DialOption
	if useInsecure {
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(insecure.NewCredentials()),
		}
	} else {
		creds := credentials.NewTLS(&tls.Config{})
		opts = []grpc.DialOption{
			grpc.WithTransportCredentials(creds),
		}
	}
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		util.GetLogger().Error("Failed to dial grpc server", zap.Error(err))
		os.Exit(1)
	}
	defer conn.Close()

	client := pb.NewTeleProxyClient(conn)

	_, err = client.Flush(context.Background(), &pb.FlushRequest{
		ApiKey: apikey,
	})
	if err != nil {
		util.GetLogger().Error("Failed to call client.Dump", zap.Error(err))
	}
}

package server

import (
	"context"
	"fmt"
	"net"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	"beleap.dev/teleproxy/pkg/teleproxy/util"
	pb "beleap.dev/teleproxy/protobuf"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/reflection"
	"google.golang.org/grpc/status"
)

var (
	_ pb.TeleProxyServer = &teleProxyServer{}
)

type teleProxyServer struct {
	pb.UnimplementedTeleProxyServer
	configs *spyconfigs.SpyConfigs

	ctx      context.Context
	cancel   context.CancelFunc
	cancelWg sync.WaitGroup

	requestChan  map[string]chan *httprequest.HttpRequestDto
	responseChan chan *httpresponse.HttpResponseDto

	apikey string

	mu sync.Mutex
}

func (s *teleProxyServer) Health(ctx context.Context, req *pb.EchoRequest) (*pb.EchoResponse, error) {
	util.GetLogger().Debug("Received Health Check")
	return &pb.EchoResponse{}, nil
}

func (s *teleProxyServer) Register(ctx context.Context, req *pb.RegisterRequest) (*pb.RegisterResponse, error) {
	if req.ApiKey != s.apikey {
		util.GetLogger().Warn("Wrong api key")
		return nil, status.Error(codes.Unauthenticated, "Wrong api key")
	}

	config := spyconfig.New(req.HeaderKey, req.HeaderValue)
	s.configs.AddSpyConfig(config)

	return &pb.RegisterResponse{
		Id: config.Id,
	}, nil
}

func (s *teleProxyServer) Deregister(ctx context.Context, req *pb.DeregisterRequest) (*pb.DeregisterResponse, error) {
	if req.ApiKey != s.apikey {
		util.GetLogger().Warn("Wrong api key")
		return nil, status.Error(codes.Unauthenticated, "Wrong api key")
	}

	s.configs.RemoveSpyConfig(req.Id)

	return &pb.DeregisterResponse{}, nil
}

func (s *teleProxyServer) Listen(stream pb.TeleProxy_ListenServer) error {
	initResp, err := stream.Recv()
	if err != nil {
		util.GetLogger().Error(
			"Failed to get request",
			zap.Error(err),
		)
		return status.Error(codes.Internal, "")
	}
	if initResp.ApiKey != s.apikey {
		util.GetLogger().Warn("Wrong api key")
		return status.Error(codes.Unauthenticated, "Wrong api key")
	}

	s.cancelWg.Add(1)
	defer s.cancelWg.Done()

	requestChan := make(chan *httprequest.HttpRequestDto)
	s.mu.Lock()
	s.requestChan[initResp.Id] = requestChan
	s.mu.Unlock()

	for {
		select {
		case <-s.ctx.Done():
			util.GetLogger().Info("Flushed" + initResp.Id)
			return status.Error(codes.Aborted, "Flushed")
		case request := <-requestChan:
			util.GetLogger().Debug("Handling spying request to " + initResp.Id)
			stream.Send(request.ToPb())
			resp, err := stream.Recv()
			if err != nil {
				util.GetLogger().Error(
					"Failed to get response",
					zap.Error(err),
				)
			}

			s.responseChan <- httpresponse.FromPb(resp)
		}
	}
}

func (s *teleProxyServer) Dump(ctx context.Context, req *pb.DumpRequest) (*pb.DumpResponse, error) {
	if req.ApiKey != s.apikey {
		util.GetLogger().Warn("Wrong api key")
		return nil, status.Error(codes.Unauthenticated, "Wrong api key")
	}

	res, err := s.configs.DumpSpyConfigs()
	if err != nil {
		util.GetLogger().Error(
			"Failed to dump spy configs",
			zap.Error(err),
		)
		return nil, status.Error(codes.Internal, "Failed to dump spy configs")
	}
	resp := &pb.DumpResponse{
		Dump: res,
	}
	return resp, nil
}

func (s *teleProxyServer) Flush(ctx context.Context, req *pb.FlushRequest) (*pb.FlushResponse, error) {
	if req.ApiKey != s.apikey {
		util.GetLogger().Warn("Wrong api key")
		return nil, status.Error(codes.Unauthenticated, "Wrong api key")
	}

	s.configs.FlushSpyConfigs()
	s.cancel()
	s.cancelWg.Wait()

	ctx, cancel := context.WithCancel(context.Background())
	s.ctx = ctx
	s.cancel = cancel
	return &pb.FlushResponse{}, nil
}

func Start(requestChan map[string]chan *httprequest.HttpRequestDto, responseChan chan *httpresponse.HttpResponseDto, configs *spyconfigs.SpyConfigs, port int, apikey string) {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		util.GetLogger().Error(
			"Failed to start server",
			zap.Error(err),
		)
		return
	}

	grpcServer := grpc.NewServer()

	ctx, cancel := context.WithCancel(context.Background())
	serv := &teleProxyServer{
		configs: configs,

		ctx:    ctx,
		cancel: cancel,

		requestChan:  requestChan,
		responseChan: responseChan,

		apikey: apikey,
	}
	pb.RegisterTeleProxyServer(grpcServer, serv)
	util.GetLogger().Info("Listening on " + lis.Addr().String())
	reflection.Register(grpcServer)
	util.GetLogger().Error("", zap.Error(grpcServer.Serve(lis)))
}

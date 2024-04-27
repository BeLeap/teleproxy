package server

import (
	"context"
	"fmt"
	"log"
	"net"
	"os"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
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

	ctx      context.Context
	cancel   context.CancelFunc
	cancelWg sync.WaitGroup

	requestChan  chan *httprequest.HttpRequestDto
	responseChan chan *httpresponse.HttpResponseDto

	streamMap map[string]chan bool
	mu        sync.Mutex
}

func (s *teleProxyServer) Register(ctx context.Context, req *pb.RegisterRequest) (*pb.RegisterResponse, error) {
	if req.ApiKey != apiKey {
		logger.Print("Not matching api key")
		return nil, status.Error(codes.Unauthenticated, "Not matching api key")
	}

	config := spyconfig.New(req.HeaderKey, req.HeaderValue)
	s.configs.AddSpyConfig(config)

	return &pb.RegisterResponse{
		Id: config.Id,
	}, nil
}

func (s *teleProxyServer) Deregister(ctx context.Context, req *pb.DeregisterRequest) (*pb.DeregisterResponse, error) {
	if req.ApiKey != apiKey {
		logger.Print("Not matching api key")
		return nil, status.Error(codes.Unauthenticated, "Not matching api key")
	}

	s.configs.RemoveSpyConfig(req.Id)

	return &pb.DeregisterResponse{}, nil
}

func (s *teleProxyServer) Listen(stream pb.TeleProxy_ListenServer) error {
	initResp, err := stream.Recv()
	if err != nil {
		log.Printf("Failed to get request: %v", err)
		return status.Error(codes.Internal, "")
	}
	if initResp.ApiKey != apiKey {
		log.Print("Not matching api key")
		return status.Error(codes.Unauthenticated, "Not matching api key")
	}

	s.cancelWg.Add(1)
	defer s.cancelWg.Done()

	for {
		executeChan := make(chan bool)
		s.mu.Lock()
		s.streamMap[initResp.Id] = executeChan
		s.mu.Unlock()

		select {
		case <-s.ctx.Done():
			logger.Printf("Flushed %s", initResp.Id)
			return status.Error(codes.Aborted, "Flushed")
		case <-executeChan:
			request := <-s.requestChan
			stream.Send(request.ToPb())
			resp, err := stream.Recv()
			if err != nil {
				logger.Printf("Failed to get response: %v", err)
			}

			s.responseChan <- httpresponse.FromPb(resp)
		}
	}
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

func (s *teleProxyServer) Flush(ctx context.Context, req *pb.FlushRequest) (*pb.FlushResponse, error) {
	if req.ApiKey != apiKey {
		logger.Print("Not matching api key")
		return nil, status.Error(codes.Unauthenticated, "Not matching api key")
	}

	s.configs.FlushSpyConfigs()
	s.cancel()
	s.cancelWg.Wait()

	ctx, cancel := context.WithCancel(context.Background())
	s.ctx = ctx
	s.cancel = cancel
	return &pb.FlushResponse{}, nil
}

func Start(idChan chan string, requestChan chan *httprequest.HttpRequestDto, responseChan chan *httpresponse.HttpResponseDto, configs *spyconfigs.SpyConfigs, port int) {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		logger.Fatalf("Failed to start server: %v", err)
	}

	grpcServer := grpc.NewServer()

	ctx, cancel := context.WithCancel(context.Background())
	serv := &teleProxyServer{
		configs: configs,

		ctx:    ctx,
		cancel: cancel,

		requestChan:  requestChan,
		responseChan: responseChan,

		streamMap: map[string](chan bool){},
	}
	pb.RegisterTeleProxyServer(grpcServer, serv)
	logger.Printf("Listening on %s", lis.Addr().String())
	go grpcServer.Serve(lis)

	for {
		id := <-idChan
		serv.streamMap[id] <- true
	}
}

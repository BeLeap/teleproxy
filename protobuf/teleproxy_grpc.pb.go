// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v5.26.1
// source: protobuf/teleproxy.proto

package protobuf

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// TeleProxyClient is the client API for TeleProxy service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TeleProxyClient interface {
	Listen(ctx context.Context, in *ListenRequest, opts ...grpc.CallOption) (TeleProxy_ListenClient, error)
}

type teleProxyClient struct {
	cc grpc.ClientConnInterface
}

func NewTeleProxyClient(cc grpc.ClientConnInterface) TeleProxyClient {
	return &teleProxyClient{cc}
}

func (c *teleProxyClient) Listen(ctx context.Context, in *ListenRequest, opts ...grpc.CallOption) (TeleProxy_ListenClient, error) {
	stream, err := c.cc.NewStream(ctx, &TeleProxy_ServiceDesc.Streams[0], "/TeleProxy/Listen", opts...)
	if err != nil {
		return nil, err
	}
	x := &teleProxyListenClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type TeleProxy_ListenClient interface {
	Recv() (*Http, error)
	grpc.ClientStream
}

type teleProxyListenClient struct {
	grpc.ClientStream
}

func (x *teleProxyListenClient) Recv() (*Http, error) {
	m := new(Http)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// TeleProxyServer is the server API for TeleProxy service.
// All implementations must embed UnimplementedTeleProxyServer
// for forward compatibility
type TeleProxyServer interface {
	Listen(*ListenRequest, TeleProxy_ListenServer) error
	mustEmbedUnimplementedTeleProxyServer()
}

// UnimplementedTeleProxyServer must be embedded to have forward compatible implementations.
type UnimplementedTeleProxyServer struct {
}

func (UnimplementedTeleProxyServer) Listen(*ListenRequest, TeleProxy_ListenServer) error {
	return status.Errorf(codes.Unimplemented, "method Listen not implemented")
}
func (UnimplementedTeleProxyServer) mustEmbedUnimplementedTeleProxyServer() {}

// UnsafeTeleProxyServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TeleProxyServer will
// result in compilation errors.
type UnsafeTeleProxyServer interface {
	mustEmbedUnimplementedTeleProxyServer()
}

func RegisterTeleProxyServer(s grpc.ServiceRegistrar, srv TeleProxyServer) {
	s.RegisterService(&TeleProxy_ServiceDesc, srv)
}

func _TeleProxy_Listen_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(ListenRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(TeleProxyServer).Listen(m, &teleProxyListenServer{stream})
}

type TeleProxy_ListenServer interface {
	Send(*Http) error
	grpc.ServerStream
}

type teleProxyListenServer struct {
	grpc.ServerStream
}

func (x *teleProxyListenServer) Send(m *Http) error {
	return x.ServerStream.SendMsg(m)
}

// TeleProxy_ServiceDesc is the grpc.ServiceDesc for TeleProxy service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var TeleProxy_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "TeleProxy",
	HandlerType: (*TeleProxyServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Listen",
			Handler:       _TeleProxy_Listen_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "protobuf/teleproxy.proto",
}

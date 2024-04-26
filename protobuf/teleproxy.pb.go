// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.28.1
// 	protoc        v5.26.1
// source: protobuf/teleproxy.proto

package protobuf

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type ListenRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ApiKey      string `protobuf:"bytes,1,opt,name=api_key,json=apiKey,proto3" json:"api_key,omitempty"`
	HeaderKey   string `protobuf:"bytes,2,opt,name=header_key,json=headerKey,proto3" json:"header_key,omitempty"`
	HeaderValue string `protobuf:"bytes,3,opt,name=header_value,json=headerValue,proto3" json:"header_value,omitempty"`
}

func (x *ListenRequest) Reset() {
	*x = ListenRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_teleproxy_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListenRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListenRequest) ProtoMessage() {}

func (x *ListenRequest) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_teleproxy_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListenRequest.ProtoReflect.Descriptor instead.
func (*ListenRequest) Descriptor() ([]byte, []int) {
	return file_protobuf_teleproxy_proto_rawDescGZIP(), []int{0}
}

func (x *ListenRequest) GetApiKey() string {
	if x != nil {
		return x.ApiKey
	}
	return ""
}

func (x *ListenRequest) GetHeaderKey() string {
	if x != nil {
		return x.HeaderKey
	}
	return ""
}

func (x *ListenRequest) GetHeaderValue() string {
	if x != nil {
		return x.HeaderValue
	}
	return ""
}

type Http struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Method string `protobuf:"bytes,1,opt,name=method,proto3" json:"method,omitempty"`
}

func (x *Http) Reset() {
	*x = Http{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_teleproxy_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Http) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Http) ProtoMessage() {}

func (x *Http) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_teleproxy_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Http.ProtoReflect.Descriptor instead.
func (*Http) Descriptor() ([]byte, []int) {
	return file_protobuf_teleproxy_proto_rawDescGZIP(), []int{1}
}

func (x *Http) GetMethod() string {
	if x != nil {
		return x.Method
	}
	return ""
}

type DumpRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ApiKey string `protobuf:"bytes,1,opt,name=api_key,json=apiKey,proto3" json:"api_key,omitempty"`
}

func (x *DumpRequest) Reset() {
	*x = DumpRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_teleproxy_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DumpRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DumpRequest) ProtoMessage() {}

func (x *DumpRequest) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_teleproxy_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DumpRequest.ProtoReflect.Descriptor instead.
func (*DumpRequest) Descriptor() ([]byte, []int) {
	return file_protobuf_teleproxy_proto_rawDescGZIP(), []int{2}
}

func (x *DumpRequest) GetApiKey() string {
	if x != nil {
		return x.ApiKey
	}
	return ""
}

type DumpResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Dump string `protobuf:"bytes,1,opt,name=dump,proto3" json:"dump,omitempty"`
}

func (x *DumpResponse) Reset() {
	*x = DumpResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_teleproxy_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DumpResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DumpResponse) ProtoMessage() {}

func (x *DumpResponse) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_teleproxy_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DumpResponse.ProtoReflect.Descriptor instead.
func (*DumpResponse) Descriptor() ([]byte, []int) {
	return file_protobuf_teleproxy_proto_rawDescGZIP(), []int{3}
}

func (x *DumpResponse) GetDump() string {
	if x != nil {
		return x.Dump
	}
	return ""
}

var File_protobuf_teleproxy_proto protoreflect.FileDescriptor

var file_protobuf_teleproxy_proto_rawDesc = []byte{
	0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x65, 0x6c, 0x65, 0x70,
	0x72, 0x6f, 0x78, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6a, 0x0a, 0x0d, 0x4c, 0x69,
	0x73, 0x74, 0x65, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x61,
	0x70, 0x69, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x70,
	0x69, 0x4b, 0x65, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6b,
	0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
	0x4b, 0x65, 0x79, 0x12, 0x21, 0x0a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x76, 0x61,
	0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x68, 0x65, 0x61, 0x64, 0x65,
	0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x1e, 0x0a, 0x04, 0x48, 0x74, 0x74, 0x70, 0x12, 0x16,
	0x0a, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
	0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x22, 0x26, 0x0a, 0x0b, 0x44, 0x75, 0x6d, 0x70, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x61, 0x70, 0x69, 0x5f, 0x6b, 0x65, 0x79,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x70, 0x69, 0x4b, 0x65, 0x79, 0x22, 0x22,
	0x0a, 0x0c, 0x44, 0x75, 0x6d, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12,
	0x0a, 0x04, 0x64, 0x75, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x64, 0x75,
	0x6d, 0x70, 0x32, 0x57, 0x0a, 0x09, 0x54, 0x65, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x12,
	0x23, 0x0a, 0x06, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x12, 0x0e, 0x2e, 0x4c, 0x69, 0x73, 0x74,
	0x65, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x05, 0x2e, 0x48, 0x74, 0x74, 0x70,
	0x22, 0x00, 0x30, 0x01, 0x12, 0x25, 0x0a, 0x04, 0x44, 0x75, 0x6d, 0x70, 0x12, 0x0c, 0x2e, 0x44,
	0x75, 0x6d, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0d, 0x2e, 0x44, 0x75, 0x6d,
	0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0x1f, 0x5a, 0x1d, 0x62,
	0x65, 0x6c, 0x65, 0x61, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x2f, 0x74, 0x65, 0x6c, 0x65, 0x70, 0x72,
	0x6f, 0x78, 0x79, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x62, 0x06, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_protobuf_teleproxy_proto_rawDescOnce sync.Once
	file_protobuf_teleproxy_proto_rawDescData = file_protobuf_teleproxy_proto_rawDesc
)

func file_protobuf_teleproxy_proto_rawDescGZIP() []byte {
	file_protobuf_teleproxy_proto_rawDescOnce.Do(func() {
		file_protobuf_teleproxy_proto_rawDescData = protoimpl.X.CompressGZIP(file_protobuf_teleproxy_proto_rawDescData)
	})
	return file_protobuf_teleproxy_proto_rawDescData
}

var file_protobuf_teleproxy_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_protobuf_teleproxy_proto_goTypes = []interface{}{
	(*ListenRequest)(nil), // 0: ListenRequest
	(*Http)(nil),          // 1: Http
	(*DumpRequest)(nil),   // 2: DumpRequest
	(*DumpResponse)(nil),  // 3: DumpResponse
}
var file_protobuf_teleproxy_proto_depIdxs = []int32{
	0, // 0: TeleProxy.Listen:input_type -> ListenRequest
	2, // 1: TeleProxy.Dump:input_type -> DumpRequest
	1, // 2: TeleProxy.Listen:output_type -> Http
	3, // 3: TeleProxy.Dump:output_type -> DumpResponse
	2, // [2:4] is the sub-list for method output_type
	0, // [0:2] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_protobuf_teleproxy_proto_init() }
func file_protobuf_teleproxy_proto_init() {
	if File_protobuf_teleproxy_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_protobuf_teleproxy_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListenRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_protobuf_teleproxy_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Http); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_protobuf_teleproxy_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DumpRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_protobuf_teleproxy_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DumpResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_protobuf_teleproxy_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_protobuf_teleproxy_proto_goTypes,
		DependencyIndexes: file_protobuf_teleproxy_proto_depIdxs,
		MessageInfos:      file_protobuf_teleproxy_proto_msgTypes,
	}.Build()
	File_protobuf_teleproxy_proto = out.File
	file_protobuf_teleproxy_proto_rawDesc = nil
	file_protobuf_teleproxy_proto_goTypes = nil
	file_protobuf_teleproxy_proto_depIdxs = nil
}

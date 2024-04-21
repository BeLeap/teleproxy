.PHONY: protobuf

protobuf: protobuf/teleproxy.proto
	protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    protobuf/teleproxy.proto

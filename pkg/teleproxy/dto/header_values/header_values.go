package headervalues

import (
	pb "beleap.dev/teleproxy/protobuf"
)

func FromPb(in map[string]*pb.HeaderValues) map[string][]string {
	header := map[string][]string{}
	for k, v := range in {
		header[k] = v.Values
	}

	return header
}

func ToPb(in map[string][]string) map[string]*pb.HeaderValues {
	header := map[string]*pb.HeaderValues{}
	for k, v := range in {
		header[k] = &pb.HeaderValues{
			Values: v,
		}
	}

	return header
}

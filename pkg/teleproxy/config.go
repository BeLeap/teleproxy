package teleproxy

import (
	"sync"
)

type ProxyConfig struct {
	HeaderKey string;
	HeaderValue string;

	To string;
}

type ProxyConfigs struct {
	mu sync.Mutex;
	ProxyConfigs []ProxyConfig;
}

func (c *ProxyConfigs) DumpProxyConfigs() (string, error) {
	return "[]", nil
}

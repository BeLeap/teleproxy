package config

import (
	"encoding/json"
	"log"
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

func New() ProxyConfigs {
	return ProxyConfigs{
		mu:           sync.Mutex{},
		ProxyConfigs: []ProxyConfig{},
	}
}

func (c *ProxyConfigs) DumpProxyConfigs() (string, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	buf, err := json.Marshal(c)
	if err != nil {
		log.Fatalf("Failed to marshal proxy configs: %v", err)
		return "", err
	}

	return string(buf), nil
}

func (c *ProxyConfigs) AddProxyConfigs(config ProxyConfig) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.ProxyConfigs = append(c.ProxyConfigs, config)
}

package config

import (
	"encoding/json"
	"log"
	"sync"
)

type SpyConfig struct {
	HeaderKey string;
	HeaderValue string;

	To string;
}

type SpyConfigs struct {
	mu sync.Mutex;
	SpyConfigs []SpyConfig;
}

func New() SpyConfigs {
	return SpyConfigs{
		mu:           sync.Mutex{},
		SpyConfigs: []SpyConfig{},
	}
}

func (c *SpyConfigs) DumpSpyConfigs() (string, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	buf, err := json.Marshal(c)
	if err != nil {
		log.Fatalf("Failed to marshal proxy configs: %v", err)
		return "", err
	}

	return string(buf), nil
}

func (c *SpyConfigs) AddSpyConfigs(config SpyConfig) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.SpyConfigs = append(c.SpyConfigs, config)
}

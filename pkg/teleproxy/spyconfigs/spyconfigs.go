package spyconfigs

import (
	"encoding/json"
	"log"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
)

type SpyConfigs struct {
	mu *sync.Mutex;
	SpyConfigs []spyconfig.SpyConfig;
}

func New() SpyConfigs {
	return SpyConfigs{
		mu:           &sync.Mutex{},
		SpyConfigs: []spyconfig.SpyConfig{},
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

func (c *SpyConfigs) AddSpyConfigs(config spyconfig.SpyConfig) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.SpyConfigs = append(c.SpyConfigs, config)
}

package spyconfigs

import (
	"encoding/json"
	"errors"
	"log"
	"slices"
	"sync"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
)

type SpyConfigs struct {
	mu         *sync.Mutex
	SpyConfigs []spyconfig.SpyConfig
}

func New() SpyConfigs {
	return SpyConfigs{
		mu:         &sync.Mutex{},
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

func (c *SpyConfigs) AddSpyConfig(config spyconfig.SpyConfig) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.SpyConfigs = append(c.SpyConfigs, config)
}

func (c *SpyConfigs) RemoveSpyConfig(id string) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.SpyConfigs = slices.DeleteFunc(c.SpyConfigs, func(config spyconfig.SpyConfig) bool {
		return config.Id == id
	})
}

var NoMatchingError = errors.New("No Matching Result")

func (c *SpyConfigs) GetMatching(header map[string][]string) (*string, error) {
	for key, values := range header {
		for _, config := range c.SpyConfigs {
			if config.HeaderKey == key {
				for _, value := range values {
					if config.HeaderValue == value {
						return &config.Id, nil
					}
				}
			}
		}
	}

	return nil, NoMatchingError
}

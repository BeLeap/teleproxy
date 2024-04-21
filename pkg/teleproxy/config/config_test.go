package config_test

import (
	"testing"

	"beleap.dev/teleproxy/pkg/teleproxy/config"
	"gotest.tools/assert"
)

func TestDumpProxyConfigsWithEmpty(t *testing.T) {
	pcs := config.New()
	result, err := pcs.DumpProxyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(t, result, "{\"ProxyConfigs\":[]}")
}

func TestDumpProxyConfigs(t *testing.T) {
	pcs := config.New()
	pcs.AddProxyConfigs(config.ProxyConfig{
		HeaderKey:   "Some-Header",
		HeaderValue: "SomeValue",

		To: "SomeTo",
	})
	result, err := pcs.DumpProxyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(t, result, "{\"ProxyConfigs\":[{\"HeaderKey\":\"Some-Header\",\"HeaderValue\":\"SomeValue\",\"To\":\"SomeTo\"}]}")
}

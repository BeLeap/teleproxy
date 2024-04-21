package config_test

import (
	"testing"

	"beleap.dev/teleproxy/pkg/teleproxy/config"
	"gotest.tools/assert"
)

func TestDumpProxyConfigsWithEmpty(t *testing.T) {
	pcs := config.New()
	result, err := pcs.DumpSpyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(t, result, "{\"SpyConfigs\":[]}")
}

func TestDumpProxyConfigs(t *testing.T) {
	pcs := config.New()
	pcs.AddSpyConfigs(config.SpyConfig{
		HeaderKey:   "Some-Header",
		HeaderValue: "SomeValue",

		To: "SomeTo",
	})
	result, err := pcs.DumpSpyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(t, result, "{\"SpyConfigs\":[{\"HeaderKey\":\"Some-Header\",\"HeaderValue\":\"SomeValue\",\"To\":\"SomeTo\"}]}")
}

package spyconfigs_test

import (
	"fmt"
	"testing"

	"beleap.dev/teleproxy/pkg/teleproxy/spyconfig"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	"gotest.tools/assert"
)

func TestDumpProxyConfigsWithEmpty(t *testing.T) {
	pcs := spyconfigs.New()
	result, err := pcs.DumpSpyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(t, result, "{\"SpyConfigs\":[]}")
}

func TestAddSpyConfigs(t *testing.T) {
	pcs := spyconfigs.New()

	config := spyconfig.New("Some-Header", "SomeValue", "SomeTo")
	pcs.AddSpyConfigs(config)
	result, err := pcs.DumpSpyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}

	assert.Equal(
		t,
		result,
		fmt.Sprintf(
			"{\"SpyConfigs\":[{\"Id\":\"%s\",\"HeaderKey\":\"Some-Header\",\"HeaderValue\":\"SomeValue\",\"To\":\"SomeTo\"}]}",
			config.Id,
		),
	)
}

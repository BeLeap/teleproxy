package teleproxy_test

import (
	"testing"

	"beleap.dev/teleproxy/pkg/teleproxy"
	"gotest.tools/assert"
)


func TestDumpProxyConfigs(t *testing.T) {
	pcs := teleproxy.ProxyConfigs {}
	result, err := pcs.DumpProxyConfigs()

	if err != nil {
		t.Errorf("err should not be nil: %v", err)
	}
	
	assert.Equal(t, result, "[]")
}

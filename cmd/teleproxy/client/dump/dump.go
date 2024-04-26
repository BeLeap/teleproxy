package dump

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"github.com/spf13/cobra"
)

var DumpCommand = &cobra.Command{
	Use: "dump",
	Run: func(cmd *cobra.Command, args []string) {
		client.Dump(addr, apikey)
	},
}

var addr string
var apikey string

func init() {
	DumpCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
	DumpCommand.Flags().StringVar(&apikey, "apikey", "", "api key")
}

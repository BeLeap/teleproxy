package client

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"beleap.dev/teleproxy/cmd/teleproxy/client/dump"
	"github.com/spf13/cobra"
)

var ClientCommand = &cobra.Command{
	Use: "client",
	Run: func(cmd *cobra.Command, args []string) {
		client.StartListen(addr, apikey, key, value)
	},
}

var addr string
var apikey string
var key string
var value string

func init() {
	ClientCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
	ClientCommand.Flags().StringVarP(&apikey, "apikey", "", "", "api key")
	ClientCommand.Flags().StringVarP(&key, "key", "k", "User-No", "Header Key to Spy")
	ClientCommand.Flags().StringVarP(&value, "value", "v", "", "Header Value to Spy")

	ClientCommand.AddCommand(dump.DumpCommand)
}

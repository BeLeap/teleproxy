package client

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"github.com/spf13/cobra"
)

var ClientCommand = &cobra.Command{
	Use: "client",
	Run: func(cmd *cobra.Command, args []string) {
		if dump {
			client.Dump(addr)
		} else {
			client.StartListen(addr, key, value)
		}
	},
}

var addr string
var key string
var value string
var dump bool

func init() {
	ClientCommand.Flags().BoolVarP(&dump, "dump", "d", false, "dump server spy configs")
	ClientCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
	ClientCommand.Flags().StringVarP(&key, "key", "k", "User-No", "Header Key to Spy")
	ClientCommand.Flags().StringVarP(&value, "value", "v", "", "Header Value to Spy")
}

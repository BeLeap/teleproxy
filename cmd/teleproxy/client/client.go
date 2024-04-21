package client

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"github.com/spf13/cobra"
)

var ClientCommand = &cobra.Command{
	Use: "client",
	Run: func(cmd *cobra.Command, args []string) {
		client.StartListen(addr)
	},
}

var addr string

func init() {
	ClientCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
}

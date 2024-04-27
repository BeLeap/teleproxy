package client

import (
	"context"
	"os"
	"os/signal"
	"sync"
	"syscall"

	"beleap.dev/teleproxy/cmd/teleproxy/client/admin"
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"github.com/spf13/cobra"
)

var ClientCommand = &cobra.Command{
	Use: "client",
	Run: func(cmd *cobra.Command, args []string) {
		wg := sync.WaitGroup{}
		ctx, cancel := context.WithCancel(context.Background())
		go client.StartListen(ctx, &wg, addr, apikey, key, value, target)

		quit := make(chan os.Signal, 1)
		signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)

		<-quit
		cancel()
		wg.Wait()
	},
}

var addr string
var apikey string
var key string
var value string
var target string

func init() {
	ClientCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
	ClientCommand.Flags().StringVarP(&apikey, "apikey", "", "", "api key")
	ClientCommand.Flags().StringVarP(&key, "key", "k", "User-No", "Header Key to Spy")
	ClientCommand.Flags().StringVarP(&value, "value", "v", "", "Header Value to Spy")
	ClientCommand.Flags().StringVarP(&target, "target", "t", "", "Target")

	ClientCommand.AddCommand(admin.AdminCommand)
}

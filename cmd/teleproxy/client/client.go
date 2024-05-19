package client

import (
	"context"
	"os"
	"os/signal"
	"sync"
	"syscall"

	"beleap.dev/teleproxy/cmd/teleproxy/client/admin"
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"beleap.dev/teleproxy/pkg/teleproxy/util"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var ClientCommand = &cobra.Command{
	Use: "client",
	Run: func(cmd *cobra.Command, args []string) {
		verbose := viper.GetBool("verbose")
		util.SetVerbosity(verbose)

		apikey := viper.GetString("apikey")
		addr := viper.GetString("addr")
		insecure := viper.GetBool("insecure")

		wg := sync.WaitGroup{}
		ctx, cancel := context.WithCancel(context.Background())
		go client.StartListen(ctx, &wg, addr, apikey, key, value, target, insecure)

		quit := make(chan os.Signal, 1)
		signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)

		<-quit
		cancel()
		wg.Wait()
	},
}

var key string
var value string
var target string

func init() {
	ClientCommand.PersistentFlags().StringP("addr", "a", "127.0.0.1:4001", "server addr")
	viper.BindPFlag("addr", ClientCommand.PersistentFlags().Lookup("addr"))

	ClientCommand.PersistentFlags().BoolP("insecure", "i", false, "Use insecure connection")
	viper.BindPFlag("insecure", ClientCommand.PersistentFlags().Lookup("insecure"))

	ClientCommand.Flags().StringVar(&key, "key", "User-No", "Header Key to Spy")
	ClientCommand.Flags().StringVar(&value, "value", "", "Header Value to Spy")
	ClientCommand.Flags().StringVarP(&target, "target", "t", "", "Target")

	ClientCommand.AddCommand(admin.AdminCommand)
}

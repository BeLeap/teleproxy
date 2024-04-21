package server

import (
	"log"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy/proxy"
	"beleap.dev/teleproxy/pkg/teleproxy/server"
	"beleap.dev/teleproxy/pkg/teleproxy/spyconfigs"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var ServerCommand = &cobra.Command{
	Use: "server",
	Run: func(cmd *cobra.Command, args []string) {
		port := viper.GetInt("port")
		proxyPort := viper.GetInt("proxyPort")

		configs := spyconfigs.New()

		go server.StartServer(&configs, port)
		proxy.StartProxy(&configs, proxyPort)
	},
}

var cfgFile string

func init() {
	cobra.OnInitialize(initConfig)

	ServerCommand.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "path for config file")

	ServerCommand.Flags().IntP("port", "l", 2344, "listening port")
	viper.BindPFlag("port", ServerCommand.Flags().Lookup("port"))

	ServerCommand.Flags().IntP("proxyPort", "p", 2345, "proxing port")
	viper.BindPFlag("proxyPort", ServerCommand.Flags().Lookup("proxyPort"))
}

func initConfig() {
	if cfgFile == "" {
		return
	}

	viper.SetConfigFile(cfgFile)

	if err := viper.ReadInConfig(); err != nil {
		log.Fatalln("Can't read config: ", err)
		os.Exit(1)
	}
}

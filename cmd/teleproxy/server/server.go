package server

import (
	"log"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy/dto/httprequest"
	"beleap.dev/teleproxy/pkg/teleproxy/dto/httpresponse"
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
		target := viper.GetString("target")
		proxyPort := viper.GetInt("proxyPort")
		apikey := viper.GetString("apikey")

		configs := spyconfigs.New()

		requestChan := map[string]chan *httprequest.HttpRequestDto{}
		responseChan := make(chan *httpresponse.HttpResponseDto)

		go server.Start(requestChan, responseChan, &configs, port, apikey)
		proxy.Start(requestChan, responseChan, &configs, proxyPort, target)
	},
}

var cfgFile string

func init() {
	cobra.OnInitialize(initConfig)

	ServerCommand.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "path for config file")

	ServerCommand.Flags().IntP("port", "l", 4001, "listening port")
	viper.BindPFlag("port", ServerCommand.Flags().Lookup("port"))

	ServerCommand.Flags().StringP("target", "t", "http://localhost:8080", "target")
	viper.BindPFlag("target", ServerCommand.Flags().Lookup("target"))

	ServerCommand.Flags().IntP("proxyPort", "p", 4000, "proxing port")
	viper.BindPFlag("proxyPort", ServerCommand.Flags().Lookup("proxyPort"))

	ServerCommand.Flags().StringP("apikey", "k", "", "api key for auth")
	viper.BindPFlag("apikey", ServerCommand.Flags().Lookup("apikey"))
}

func initConfig() {
	viper.AutomaticEnv()

	if cfgFile == "" {
		return
	}

	viper.SetConfigFile(cfgFile)

	if err := viper.ReadInConfig(); err != nil {
		log.Fatalln("Can't read config: ", err)
		os.Exit(1)
	}
}

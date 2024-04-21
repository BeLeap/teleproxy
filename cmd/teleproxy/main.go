package main

import (
	"log"
	"os"

	"beleap.dev/teleproxy/pkg/teleproxy"
	"beleap.dev/teleproxy/pkg/teleproxy/server"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use: "Teleproxy",
	Run: func(cmd *cobra.Command, args []string) {
		port := viper.GetInt("port")
		proxyPort := viper.GetInt("proxyPort")

		go server.StartServer(port)
		teleproxy.StartProxy(proxyPort)
	},
}

var cfgFile string

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "path for config file")

	rootCmd.Flags().IntP("port", "l", 2344, "listening port")
	viper.BindPFlag("port", rootCmd.Flags().Lookup("port"))

	rootCmd.Flags().IntP("proxyPort", "p", 2345, "proxing port")
	viper.BindPFlag("proxyPort", rootCmd.Flags().Lookup("proxyPort"))
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

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
}

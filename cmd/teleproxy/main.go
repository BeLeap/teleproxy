package main

import (
	"log"
	"os"

	"beleap.dev/teleproxy/cmd/teleproxy/client"
	"beleap.dev/teleproxy/cmd/teleproxy/server"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use: "teleproxy",
}

func init() {
	rootCmd.PersistentFlags().BoolP("verbose", "v", false, "verbose")
	viper.BindPFlag("verbose", rootCmd.PersistentFlags().Lookup("verbose"))
	rootCmd.PersistentFlags().String("apikey", "", "api key for auth")
	viper.BindPFlag("apikey", rootCmd.PersistentFlags().Lookup("apikey"))

	rootCmd.AddCommand(server.ServerCommand)
	rootCmd.AddCommand(client.ClientCommand)
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
}

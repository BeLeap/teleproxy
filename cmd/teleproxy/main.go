package main

import (
	"log"
	"os"

	"beleap.dev/teleproxy/cmd/teleproxy/client"
	"beleap.dev/teleproxy/cmd/teleproxy/server"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use: "teleproxy",
}

func init() {
	rootCmd.AddCommand(server.ServerCommand)
	rootCmd.AddCommand(client.ClientCommand)
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
		os.Exit(1)
	}
}

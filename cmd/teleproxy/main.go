package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command {
	Use: "Teleproxy",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Hello, World!")
	},
}

var cfgFile string

func init() {
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "path for config file")
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err);
		os.Exit(1);
	}
}

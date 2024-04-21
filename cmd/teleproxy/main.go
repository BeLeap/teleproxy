package main

import (
	"fmt"
	"log"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use: "Teleproxy",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Hello, World!")
		fmt.Println(viper.GetInt("port"))
	},
}

var cfgFile string

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.PersistentFlags().StringVarP(&cfgFile, "config", "c", "", "path for config file")

	rootCmd.Flags().IntP("port", "p", 2344, "listening port")
	viper.BindPFlag("port", rootCmd.Flags().Lookup("port"))
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

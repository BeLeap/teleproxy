package admin

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"github.com/spf13/cobra"
)

var AdminCommand = &cobra.Command{
	Use: "admin",
	Run: func(cmd *cobra.Command, args []string) {
		switch run {
		case "dump":
			client.Dump(addr, apikey, insecure)
			break
		case "flush":
			client.Flush(addr, apikey, insecure)
			break
		}
	},
}

var addr string
var apikey string
var run string
var insecure bool

func init() {
	AdminCommand.Flags().StringVarP(&addr, "addr", "a", "127.0.0.1:2344", "server addr")
	AdminCommand.Flags().StringVar(&apikey, "apikey", "", "api key")
	AdminCommand.Flags().StringVarP(&run, "run", "r", "", "action to run")
	AdminCommand.Flags().BoolVarP(&insecure, "insecure", "i", false, "Use insecure connection")
}

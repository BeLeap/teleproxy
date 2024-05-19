package admin

import (
	"beleap.dev/teleproxy/pkg/teleproxy/client"
	"beleap.dev/teleproxy/pkg/teleproxy/util"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var AdminCommand = &cobra.Command{
	Use: "admin",
	Run: func(cmd *cobra.Command, args []string) {
		verbose := viper.GetBool("verbose")
		util.SetVerbosity(verbose)

		apikey := viper.GetString("apikey")
		addr := viper.GetString("addr")
		insecure := viper.GetBool("insecure")

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

var run string

func init() {
	AdminCommand.Flags().StringVarP(&run, "run", "r", "", "action to run (dump, flush)")
}

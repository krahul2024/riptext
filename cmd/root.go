package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
    depth int
    recursive bool 
)

var rootCmd = &cobra.Command{
    Use: "rt",
    Short : "riptext is a CLI Tool for working with text, files and directories", 
    Run : func(cmd *cobra.Command, args []string){
        fmt.Println("Welcome to the riptext CLI, Use rt --help for Usage")
    },
}

func init(){
    rootCmd.PersistentFlags().BoolVarP(&recursive, "recursive", "r", false, "search recursively through all the files in mentioned directory")
    rootCmd.PersistentFlags().IntVarP(&depth, "depth", "d", 0, "navigate upto that much depth recursively for target directory")
}

func Execute(){
    if err := rootCmd.Execute(); err != nil {
        fmt.Println(err) 
        os.Exit(1) 
    }
}

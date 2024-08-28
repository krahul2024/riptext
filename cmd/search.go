package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
    caseSensitive bool
    wholeWord bool
    numResults int
)

var searchCmd = &cobra.Command{
    Use : "search [pattern] [path_to_directory/file]", 
    Short : "search for any kind of files, directories, terms in a file and more", 
    Long : "search for pattern in the specified path/file, with options like case sensitivity, whole/partial word matching, strict matching etc.",
    Args : func(cmd *cobra.Command, args []string) error {
        if len(args) < 2 {
            return fmt.Errorf("arguments missing : [pattern] [path/filepath]") 
        }
        return nil 
    },
    Run : func(cmd *cobra.Command, args []string){
        pattern := args[0] 
        path := args[1] 
        search(pattern, path)
    },
}

func init(){
    searchCmd.Flags().BoolVarP(&caseSensitive, "ignore-case", "i", false, "case sensitive search results")
    searchCmd.Flags().BoolVarP(&wholeWord, "whole", "w", false, "match whole word, or contains parts of words too")
    searchCmd.Flags().IntVarP(&numResults, "limit", "n", 0, "number of results to display")
    rootCmd.AddCommand(searchCmd)
}


func search(term, path string){
    fmt.Println("Searching for related things for ", term, path, depth, recursive) 
}



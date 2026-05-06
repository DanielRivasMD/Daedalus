/*
Copyright © 2024 Daniel Rivas <danielrivasmd@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
package cmd

////////////////////////////////////////////////////////////////////////////////////////////////////

import (
	"embed"
	"sync"

	"github.com/DanielRivasMD/domovoi"
	"github.com/DanielRivasMD/horus"
	"github.com/spf13/cobra"
)

////////////////////////////////////////////////////////////////////////////////////////////////////

//go:embed docs.json
var docsFS embed.FS

////////////////////////////////////////////////////////////////////////////////////////////////////

const (
	APP     = "mbombo"
	VERSION = "v2.1.0"
	AUTHOR  = "Daniel Rivas"
	EMAIL   = "<danielrivasmd@gmail.com>"
)

////////////////////////////////////////////////////////////////////////////////////////////////////

func InitDocs() {
	info := domovoi.AppInfo{
		Name:    APP,
		Version: VERSION,
		Author:  AUTHOR,
		Email:   EMAIL,
	}
	domovoi.SetGlobalDocsConfig(docsFS, info)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func GetRootCmd() *cobra.Command {
	onceRoot.Do(func() {
		cmd := horus.Must(horus.Must(domovoi.GlobalDocs()).MakeCmd("root", runRoot))

		cmd.Flags().StringVarP(&rootFlags.inPath, "in", "", "", "items to be forged")
		cmd.Flags().StringVarP(&rootFlags.outPath, "out", "", "", "path to forge")
		cmd.Flags().StringArrayVarP(&rootFlags.inFiles, "files", "", []string{}, "forge components")
		cmd.Flags().VarP(&replacePairs, "replace", "", "replacement in form old=new, space-separated")

		horus.CheckErr(cmd.MarkFlagRequired("out"))
		horus.CheckErr(cmd.MarkFlagRequired("files"))

		cmd.PersistentFlags().BoolVarP(&rootFlags.verbose, "verbose", "v", false, "Enable verbose diagnostics")
		cmd.Version = VERSION

		rootCmd = cmd
	})
	return rootCmd
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func Execute() {
	horus.CheckErr(GetRootCmd().Execute())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

type rootFlag struct {
	verbose bool
	inPath  string
	outPath string
	outFile string
	inFiles []string
}

var (
	onceRoot  sync.Once
	rootCmd   *cobra.Command
	rootFlags rootFlag
)

////////////////////////////////////////////////////////////////////////////////////////////////////

func BuildCommands() {
	root := GetRootCmd()
	root.AddCommand(
		CompletionCmd(),
		IdentityCmd(),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runRoot(cmd *cobra.Command, args []string) {
	op := "mbombo.root"

	normalizeForgeOptions(&rootFlags)

	horus.CheckErr(
		catFiles(rootFlags),
		horus.WithOp(op),
		horus.WithMessage("Error during concatenation execution"),
		horus.WithExitCode(2),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

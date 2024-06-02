package config

import (
	"github.com/hellflame/argparse"
	"github.com/pkg/errors"
)

type (
	Opts struct {
		Args   []string
		Config string
		Pwd    string
	}
)

func GetOpts() (Opts, error) {
	parser := argparse.NewParser("projector", "gets all the values", &argparse.ParserConfig{
		DisableDefaultShowHelp: true,

		// Usage:          "",
		// EpiLog:         "",
		// DisableHelp:    false,
		// ContinueOnHelp: false,
		// DefaultAction: func() {
		// },
		// AddShellCompletion: false,
		// WithHint:           false,
		// MaxHeaderLength:    0,
		// WithColor:          false,
		// EnsureColor:        false,
		// ColorSchema:        &argparse.ColorSchema{},
	})

	args := parser.Strings("a", "args", &argparse.Option{
		Positional: true,

		// Meta:        "",
		// Default:     "",
		// Required:    false,
		// HideEntry:   false,
		// Help:        "",
		// NoHint:      false,
		// HintInfo:    "",
		// Group:       "",
		// Inheritable: false,
		// Action: func(args []string) error {
		// },
		// Choices: []interface{}{},
		// Validate: func(arg string) error {
		// },
		// Formatter: func(arg string) (interface{}, error) {
		// },
		// BindParsers: []*argparse.Parser{},
	})
	config := parser.String("c", "config", &argparse.Option{})
	pwd := parser.String("p", "pwd", &argparse.Option{})

	if err := parser.Parse(nil); err != nil {
		return Opts{}, errors.Wrap(err, "parser.Parse")
	}

	return Opts{Args: *args, Config: *config, Pwd: *pwd}, nil
}

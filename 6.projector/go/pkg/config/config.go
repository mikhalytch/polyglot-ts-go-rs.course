package config

import (
	"fmt"
	"os"
	"path"

	"go.uber.org/multierr"
)

const (
	Print Operation = iota
	Add
	Remove
)

type (
	Operation int
	Config    struct {
		Args      []string
		Operation Operation
		Pwd       string
		Config    string
	}
)

func getPwd(opts Opts) (string, error) {
	if len(opts.Pwd) != 0 {
		return opts.Pwd, nil
	}

	return os.Getwd()
}
func getConfig(opts Opts) (string, error) {
	if len(opts.Pwd) != 0 {
		return opts.Pwd, nil
	}

	if cfgDir, err := os.UserConfigDir(); err == nil {
		return path.Join(cfgDir, "projector", "projector.json"), nil
	} else if cfgDif, err2 := os.UserHomeDir(); err2 == nil {
		return path.Join(cfgDif, ".projector.json"), nil
	} else {
		return "", multierr.Append(err, err2)
	}
}
func getOperation(opts Opts) Operation {
	switch args := opts.Args; {
	case len(args) == 0:
		return Print
	case args[0] == "add":
		return Add
	case args[0] == "rm":
		return Remove
	default:
		return Print
	}
}
func getArgs(opts Opts) ([]string, error) {
	args := opts.Args
	switch op := getOperation(opts); op {
	case Add:
		if len(args) != 3 {
			return nil, fmt.Errorf("add expected 2 arguments, got %d", len(args)-1)
		}
	case Remove:
		if len(args) != 2 {
			return nil, fmt.Errorf("remove expected 1 argument, got %d", len(args)-1)
		}
	default:
		if len(args) > 1 {
			return nil, fmt.Errorf("print requires 0 to 1 args, got %d", len(args)-1)
		}
		return args, nil
	}
	return args[1:], nil
}
func NewConfig(opts Opts) (Config, error) {
	pwd, err := getPwd(opts)
	if err != nil {
		return Config{}, err
	}

	config, err := getConfig(opts)
	if err != nil {
		return Config{}, err
	}

	args, err := getArgs(opts)
	if err != nil {
		return Config{}, err
	}

	return Config{
		Args:      args,
		Operation: getOperation(opts),
		Pwd:       pwd,
		Config:    config,
	}, err
}

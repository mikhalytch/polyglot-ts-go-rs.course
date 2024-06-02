package config_test

import (
	"projector/pkg/config"
	"reflect"
	"testing"
)

func getOpts(args []string) config.Opts {
	return config.Opts{
		Args: args,
	}
}

func TestNewConfig(t *testing.T) {
	t.Run("args", func(t *testing.T) {

		testConfig := func(t *testing.T, args []string, wantArgs []string, wantOp config.Operation) {
			opts := getOpts(args)

			cfg, err := config.NewConfig(opts)
			if err != nil {
				t.Errorf("unexpected err: %+v", err)
			}

			if cfg.Operation != wantOp {
				t.Errorf("unexpected operation: want %+v, got %+v", wantOp, cfg.Operation)
			}

			if !reflect.DeepEqual(cfg.Args, wantArgs) {
				t.Errorf("unexpected cfg args: want %+v, got %+v", wantArgs, cfg.Args)
			}
		}

		t.Run("empty", func(t *testing.T) {

			tests := []struct {
				name string
				args []string
			}{
				{"nil", nil},
				{"empty", []string{}},
			}
			for _, test := range tests {
				t.Run(test.name, func(t *testing.T) {
					testConfig(t, test.args, test.args, config.Print)
				})
			}

		})

		t.Run("print key", func(t *testing.T) {
			testConfig(t, []string{"foo"}, []string{"foo"}, config.Print)
		})

		t.Run("add key value", func(t *testing.T) {
			testConfig(t, []string{"add", "foo", "bar"}, []string{"foo", "bar"}, config.Add)
		})

		t.Run("rm key", func(t *testing.T) {
			testConfig(t, []string{"rm", "bar"}, []string{"bar"}, config.Remove)
		})
	})
}

package main

import (
	"fmt"
	"github.com/pkg/errors"
)

func returnsError(value int) error {
	return fmt.Errorf("This is an error with value %d", value)
}

type (
	Foo struct {
	}
)

func CreateFoo(fail bool) (*Foo, error) {
	if fail {
		return nil, errors.New("unable to create a Foo")
	}
	return &Foo{}, nil
}

func (f *Foo) errorOnFoo(value string) error {
	return errors.Errorf("error on foo: %q", value)
}

func main() {
	err := returnsError(5)
	fmt.Println(err)

	foo, err := CreateFoo(true)
	println(foo, err.Error())

	println(foo.errorOnFoo("after Create").Error())
}

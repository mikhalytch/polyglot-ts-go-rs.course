package projector_test

import (
	"projector/pkg/config"
	"projector/pkg/projector"
	"testing"
)

func getData() projector.Data {
	return projector.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"fem": "is great",
			},
			"/foo":     {"foo": "bar2"},
			"/foo/bar": {"foo": "bar3"},
		},
	}
}

func getProjector(pwd string, data projector.Data) projector.Projector {
	return projector.NewProjector(
		config.Config{
			Args:      []string{},
			Operation: config.Print,
			Pwd:       pwd,
			Config:    `hi there, gophers!`,
		},
		data,
	)
}

// -----

func assertOk[T any](t *testing.T) func(value T, ok bool) T {
	return func(value T, ok bool) T {
		if !ok {
			t.Errorf("should be ok")
		}
		return value
	}
}
func assertFalse(t *testing.T, b bool) {
	if b {
		t.Error("expected to be false, but got true")
	}
}
func assertEq[T comparable](t *testing.T, want, got T) {
	if want != got {
		t.Errorf("want %+v, got %+v", want, got)
	}
}

// -----

func TestGetValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	assertEq(t, "bar3", assertOk[string](t)(proj.GetValue("foo")))
	assertEq(t, "is great", assertOk[string](t)(proj.GetValue("fem")))
}
func TestSetValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	assertEq(t, "bar3", assertOk[string](t)(proj.GetValue("foo")))
	proj.SetValue("foo", "bar4")
	assertEq(t, "bar4", assertOk[string](t)(proj.GetValue("foo")))
	_, ok := proj.GetValue("baz")
	assertFalse(t, ok)

	// override parent dir value
	proj.SetValue("fem", "is super great")
	assertEq(t, "is super great", assertOk[string](t)(proj.GetValue("fem")))

	// recheck /
	proj = getProjector("/", data)
	assertEq(t, "is great", assertOk[string](t)(proj.GetValue("fem")))
}
func TestRemoveValue(t *testing.T) {
	data := getData()
	proj := getProjector("/foo/bar", data)

	assertEq(t, "bar3", assertOk[string](t)(proj.GetValue("foo")))
	proj.RemoveValue("foo")
	assertEq(t, "bar2", assertOk[string](t)(proj.GetValue("foo")))
	proj.RemoveValue("fem")
	assertEq(t, "is great", assertOk[string](t)(proj.GetValue("fem")))
}

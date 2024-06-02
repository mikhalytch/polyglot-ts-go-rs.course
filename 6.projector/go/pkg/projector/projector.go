package projector

import (
	"encoding/json"
	"errors"
	pkgErrors "github.com/pkg/errors"
	"io/fs"
	"os"
	"path"
	"projector/pkg/config"
)

type (
	Data struct {
		Projector map[string]map[string]string `json:"projector"` // pwd -> key -> value
	}
	Projector struct {
		cfg  config.Config
		data Data
	}
)

func NewProjector(cfg config.Config, data Data) Projector { return Projector{cfg: cfg, data: data} }
func defaultProjector(cfg config.Config) Projector {
	return NewProjector(cfg, Data{
		Projector: make(map[string]map[string]string),
	})
}

// ProjectorFromConfig returns Projector, with the default data in case of json read error
func ProjectorFromConfig(cfg config.Config) Projector {
	switch fContents, err := os.ReadFile(cfg.Config); {
	case err == nil: // file exist and successfully read
		var data Data
		err := json.Unmarshal(fContents, &data)
		if err != nil {
			return defaultProjector(cfg)
		}
		return NewProjector(cfg, data)
	case errors.Is(err, fs.ErrNotExist): // file does not exist
		/* nop */
	default: // file cannot be read due to some error
		/* nop */
	}
	return defaultProjector(cfg)
}

// -----

func (p Projector) GetValue(key string) (string, bool) {
	curr := p.cfg.Pwd
	prev := ""

	out, found := "", false
	for curr != prev {

		if dir, ok := p.data.Projector[curr]; ok {
			if v, ok := dir[key]; ok {
				out = v
				found = true
				break
			}
		}

		prev = curr
		curr = path.Dir(curr)
	}

	return out, found
}

func (p Projector) GetValueAll() map[string]string {
	curr := p.cfg.Pwd
	prev := ""
	paths := make([]string, 0)
	for curr != prev {
		paths = append(paths, curr)
		prev = curr
		curr = path.Dir(curr)
	}

	out := make(map[string]string)

	// slices.Reverse(paths)
	// for path := range paths { }

	for i := len(paths) - 1; i >= 0; i-- {
		path := paths[i]
		if dir, ok := p.data.Projector[path]; ok {

			// maps.Copy(out, dir)
			for k, v := range dir {
				out[k] = v
			}
		}
	}

	return out
}

func (p Projector) SetValue(key, value string) {
	pwd := p.cfg.Pwd
	if _, ok := p.data.Projector[pwd]; !ok {
		p.data.Projector[pwd] = make(map[string]string)
	}
	p.data.Projector[pwd][key] = value
}

func (p Projector) RemoveValue(key string) {
	pwd := p.cfg.Pwd
	if dir, ok := p.data.Projector[pwd]; ok {
		delete(dir, key)
	}
}

func (p Projector) Save() error {
	dir := path.Dir(p.cfg.Config)
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return pkgErrors.Wrapf(err, "unable to mkdir %q", dir)
		}
	}
	data, err := json.Marshal(p.data)
	if err != nil {
		return pkgErrors.Wrap(err, `unable to json.Marshal the data`)
	}
	return os.WriteFile(p.cfg.Config, data, 0755)
}

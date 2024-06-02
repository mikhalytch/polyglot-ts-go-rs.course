package main

import (
	"encoding/json"
	"fmt"
	"log"
	"projector/pkg/config"
	"projector/pkg/projector"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		log.Fatalf("unable to getOpts; %+v", err)
	}
	// fmt.Printf("opts: %+v\n", opts)

	cfg, err := config.NewConfig(opts)
	if err != nil {
		log.Fatalf("unable to construct config from opts; %+v", err)
	}

	proj := projector.ProjectorFromConfig(cfg)
	switch op := cfg.Operation; op {
	case config.Remove, config.Add:
		if op == config.Add {
			proj.SetValue(cfg.Args[0], cfg.Args[1])
		} else {
			proj.RemoveValue(cfg.Args[0])
		}
		if err := proj.Save(); err != nil {
			log.Fatalf("error saving; %+v", err)
		}
	case config.Print:
		if len(cfg.Args) == 0 {
			data := proj.GetValueAll()
			b, err := json.Marshal(data)
			if err != nil {
				log.Fatalf("unable to marshal data %+v; %+v\n", data, err)
			}
			fmt.Printf("%+v\n", string(b))
		} else if value, ok := proj.GetValue(cfg.Args[0]); ok {
			fmt.Println(value)
		}
	default:
		log.Fatalln("unknown operation")
	}
}

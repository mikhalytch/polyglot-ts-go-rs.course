package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type (
	movement struct{ x, y int }
)

func (m *movement) add(a movement) { m.x += a.x; m.y += a.y }

// -----

func main() {
	input := getInput()
	// fmt.Printf("%s\n", input)
	lines := strings.Split(input, "\n")

	point := movement{x: 0, y: 0}
	for _, line := range lines {
		point.add(parseLine(line))
	}
	log.Println(fmt.Sprintf("result, %d, %+v", point.x*point.y, point))
}
func parseLine(line string) movement {
	parts := strings.Split(line, " ")
	amount, err := strconv.Atoi(parts[1])
	if err != nil {
		log.Fatal("wrong input: %w", err)
	}
	switch parts[0] {
	case "forward":
		return movement{x: amount, y: 0}
	case "up":
		return movement{x: 0, y: amount}
	default:
		return movement{x: 0, y: -amount}
	}
}

func getInput() string {
	return `forward 5 
down 5 
forward 8 
up 3 
down 8 
forward 2 `
}

package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/pkg/errors"
)

func getInput() string {
	return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`
}

type (
	Point struct{ x, y int }
	Line  struct{ p1, p2 Point }
)

func parsePoint(p string) (*Point, error) {
	xy := strings.Split(p, ",")
	if len(xy) != 2 {
		return nil, errors.Errorf("wrong point: %q", p)
	}
	x, err := strconv.Atoi(xy[0])
	if err != nil {
		return nil, errors.Wrap(err, "parse x")
	}
	y, err := strconv.Atoi(xy[1])
	if err != nil {
		return nil, errors.Wrap(err, "parse y")
	}
	return &Point{x: x, y: y}, nil
}
func parseLine(line string) (*Line, error) {
	pp := strings.Split(line, " -> ")
	if len(pp) != 2 {
		return nil, errors.Errorf("wrong line to parse: %q", line)
	}
	p1, err := parsePoint(pp[0])
	if err != nil {
		return nil, errors.Wrap(err, "p1")
	}
	p2, err := parsePoint(pp[1])
	if err != nil {
		return nil, errors.Wrap(err, "p2")
	}
	return &Line{p1: *p1, p2: *p2}, err
}
func (l Line) isNonSloping() bool { return l.p1.x == l.p2.x || l.p1.y == l.p2.y }
func main() {
	lines := strings.Split(getInput(), "\n")
	for lineIdx, l := range lines {
		line, err := parseLine(l)
		if err != nil {
			log.Fatalln(fmt.Sprintf("error in line %d, %+v", lineIdx, err))
		}
		if line.isNonSloping() {
			fmt.Printf("%+v\n", line)
		}
	}
}

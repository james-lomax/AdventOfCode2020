package main

import (
	"fmt"
	"regexp"
	"strconv"
	"io/ioutil"
)

type Action struct {
	a byte
	v int
}

func parse(s string) []Action {
	var actions []Action

	// What a mouthfull - just to collect regex matches...
	pat := regexp.MustCompile("([NESWLRF])([0-9]+)")
	matches := pat.FindAllStringSubmatch(s, -1)
	for _, m := range matches {
		// No chance of error on [0-9]+
		i, _ := strconv.Atoi(m[2])
		actions = append(actions, Action {
			a: m[1][0],
			v: i,
		})
	}

	for _, action := range actions {
		if (action.a == 'R' || action.a == 'L') && action.v % 90 != 0 {
			panic(fmt.Sprintf("Not expecting non-90 deg turns! (%d)", action.v))
		}
	}

	return actions
}

func direction(d int) (int, int) {
	switch d {
	case 0:
		return 0, 1
	case 1:
		return 1, 0
	case 2:
		return 0, -1
	case 3:
		return -1, 0
	default:
		panic(fmt.Sprintf("wth: d=%d", d))
	}
}

func Absi(i int) int {
	if i < 0 {
		return i*-1
	} else {
		return i
	}
}

func rotateDir(dir int, angle int) int {
	d := (dir + angle / 90) % 4
	if d < 0 {
		d = 4 + d
	}
	return d
}

func part1move(actions []Action) int {
	x := 0
	y := 0
	d := 1 // 0 = north, 1 = east, etc

	for _, action := range actions {
		switch action.a {
		case 'N':
			y += action.v
		case 'E':
			x += action.v
		case 'S':
			y -= action.v
		case 'W':
			x -= action.v
		case 'R':
			d = rotateDir(d, action.v)
		case 'L':
			d = rotateDir(d, -action.v)
		case 'F':
			dx, dy := direction(d)
			x += dx*action.v
			y += dy*action.v
		default:
			panic("Wtf")
		}
	}

	return Absi(x) + Absi(y)
}

func rotate(dx int, dy int, angle int) (int, int) {
	a := angle / 90
	if a < 0 {
		a = 4 + a
	}

	for i := 0; i < a; i++ {
		tmpDx := dx
		dx = dy
		dy = -tmpDx
	}

	return dx, dy
}

func part2move(actions []Action) int {
	x := 0
	y := 0
	wx := 10
	wy := 1
	
	for _, action := range actions {
		switch action.a {
		case 'N':
			wy += action.v
		case 'E':
			wx += action.v
		case 'S':
			wy -= action.v
		case 'W':
			wx -= action.v
		case 'R':
			wx, wy = rotate(wx, wy, action.v)
		case 'L':
			wx, wy = rotate(wx, wy, -action.v)
		case 'F':
			x += wx*action.v
			y += wy*action.v
		default:
			panic("Wtf")
		}
	}

	return Absi(x) + Absi(y)
}

func main() {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("Couldnt read the input.txt file")
	}

	actions := parse(string(content))
	m := part1move(actions)
	fmt.Printf("Part 1: %d\n", m)

	m2 := part2move(actions)
	fmt.Printf("Part 2: %d\n", m2)
}

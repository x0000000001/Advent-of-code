package main

import (
	"strings"
)

const (
	Rock     = 0
	Paper    = 1
	Scissors = 2
)

type InputType [][]int
type RetType = int

func format(lines []string) InputType {
	res := make([][]int, len(lines))

	for i, l := range lines {
		words := strings.Fields(l)
		var p0, p1 int

		switch words[0] {
		case "A":
			p0 = Rock
		case "B":
			p0 = Paper
		case "C":
			p0 = Scissors
		}

		switch words[1] {
		case "X":
			p1 = Rock
		case "Y":
			p1 = Paper
		case "Z":
			p1 = Scissors
		}

		res[i] = []int{p0, p1}[:]
	}

	return res
}

// score for p1
func score(p0 int, p1 int) int {
	sum := p1 + 1

	if p0 == p1 {
		sum += 3
	} else if p1 == (p0+1)%3 {
		sum += 6
	}

	return sum
}

func part1(rounds InputType) RetType {
	sum := 0

	for _, v := range rounds {
		sum += score(v[0], v[1])
	}

	return sum
}

func part2(rounds InputType) RetType {
	sum := 0

	for _, v := range rounds {
		var p int

		switch v[1] {
		case 0:
			p = (v[0] + 2) % 3
		case 1:
			p = v[0]
		case 2:
			p = (v[0] + 1) % 3
		}

		sum += score(v[0], p)
	}

	return sum
}

package main

import (
	"strconv"
	"strings"
)

type Range struct {
	min int
	max int
}

type InputType [][]Range
type RetType = int

func get_range(s string) Range {
	words := strings.Split(s, "-")
	min, _ := strconv.Atoi(words[0])
	max, _ := strconv.Atoi(words[1])
	return Range{
		min,
		max,
	}
}

func format(lines []string) InputType {
	res := make([][]Range, len(lines))

	for i, v := range lines {
		words := strings.Split(v, ",")
		res[i] = []Range{get_range(words[0]), get_range(words[1])}
	}

	return res
}

func part1(input InputType) RetType {
	count := 0

	for _, pair := range input {
		if (pair[0].min <= pair[1].min && pair[0].max >= pair[1].max) ||
			(pair[1].min <= pair[0].min && pair[1].max >= pair[0].max) {
			count++
		}
	}

	return count
}

func part2(input InputType) RetType {
	count := 0

	for _, pair := range input {
		if (pair[0].max >= pair[1].min && pair[0].min <= pair[1].max) || (pair[0].min <= pair[1].max && pair[0].max >= pair[1].min) {
			count++
		}
	}

	return count
}

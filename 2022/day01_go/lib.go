package main

import (
	"sort"
	"strconv"
)

type InputType [][]int
type RetType = int

func format(lines []string) InputType {
	res := make(InputType, 0)
	current := make([]int, 0)

	for _, v := range lines {
		if v == "" {
			res = append(res, current)
			current = make([]int, 0)
		} else {
			i, _ := strconv.Atoi(v)
			current = append(current, i)
		}
	}

	res = append(res, current)

	return res
}

func part1(input InputType) RetType {
	max := 0

	for _, v := range input {
		sum := 0
		for _, k := range v {
			sum += k
		}

		if sum > max {
			max = sum
		}
	}

	return max
}

func part2(input InputType) RetType {
	maxs := make([]int, 3)

	for _, v := range input {
		sum := 0
		for _, k := range v {
			sum += k
		}

		maxs = append(maxs, sum)
		sort.Ints(maxs)
		maxs = maxs[1:]
	}

	return maxs[0] + maxs[1] + maxs[2]
}

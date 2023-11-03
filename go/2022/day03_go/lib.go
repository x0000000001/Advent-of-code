package main

type InputType [][]int
type RetType = int

func get_code(c rune) int {
	if c < 97 {
		return int(c) - 38
	} else {
		return int(c) - 96
	}
}

func format(lines []string) InputType {
	res := make([][]int, len(lines))

	for i, l := range lines {
		rucksack := make([]int, len(l))
		runes := []rune(l)

		for j, c := range runes {
			rucksack[j] = get_code(c)
		}

		res[i] = rucksack
	}

	return res
}

type Set = map[int]bool

func intersect(s0 Set, s1 Set) (inter Set) {
	inter = make(Set)

	for k0 := range s0 {
		for k1 := range s1 {
			if k0 == k1 {
				inter[k0] = true
				break
			}
		}
	}

	return
}

func set_from_list(l []int) (res Set) {
	res = make(Set)

	for _, k := range l {
		res[k] = true
	}

	return
}

func part1(input InputType) RetType {
	sum := 0

	for _, l := range input {
		compartement0 := set_from_list(l[:len(l)/2])
		compartement1 := set_from_list(l[len(l)/2:])

		// the inter should contain only 1 value
		for k := range intersect(compartement0, compartement1) {
			sum += k
		}
	}

	return sum
}

func part2(input InputType) RetType {
	sum := 0

	for i := 0; i < len(input)/3; i++ {
		// again, the inter should contain only 1 value
		p0, p1, p2 := set_from_list(input[3*i]), set_from_list(input[3*i+1]), set_from_list(input[3*i+2])
		for k := range intersect(p0, intersect(p1, p2)) {
			sum += k
		}
	}

	return sum
}

package main

type InputType []rune
type RetType = int

func format(lines []string) InputType {
	return []rune(lines[0])
}

func detect_first_n_distinct(input InputType, n int) int {
	chars := make(map[rune]int)
	for i := 0; i < len(input); i++ {
		if i >= n {
			v, _ := chars[input[i-n]]
			if v > 1 {
				chars[input[i-n]] = v - 1
			} else {
				delete(chars, input[i-n])
			}
		}

		v, present := chars[input[i]]

		if present {
			chars[input[i]] = v + 1
		} else {
			chars[input[i]] = 1
			if len(chars) == n {
				return i + 1
			}
		}

	}
	return 0
}

func part1(input InputType) RetType {
	return detect_first_n_distinct(input, 4)
}

func part2(input InputType) RetType {
	return detect_first_n_distinct(input, 14)
}

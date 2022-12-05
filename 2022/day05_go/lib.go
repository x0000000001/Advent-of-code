package main

import (
	"fmt"
	"strconv"
	"strings"
)

type InputType struct {
	stacks []Stack
	moves  [][3]int
}

type RetType = string

type Stack struct {
	value rune
	next  *Stack
}

func (s *Stack) push(value rune) {
	if s == nil {
		*s = Stack{
			value: value,
			next:  nil,
		}
	} else {
		new_s := Stack{
			value: s.value,
			next:  s.next,
		}

		s.value = value
		s.next = &new_s
	}
}

func (s *Stack) print() {
	fmt.Printf("[%c] ", s.value)

	if s.next != nil {
		s.next.print()
	} else {
		fmt.Printf("\n")
	}
}

func (s *Stack) pop() rune {
	if s == nil {
		panic("can't popout of empty stack")
	} else {
		ret := s.value
		if s.next != nil {
			*s = *s.next
		}
		return ret
	}
}

func format(lines []string) InputType {
	i := 0
	for len(strings.Split(lines[i], "[")) != 1 {
		i++
	}

	stacks_count := len(strings.Fields(lines[i]))
	stacks := make([]Stack, stacks_count)
	begin := i + 1
	i--

	for i >= 0 {
		chars := []rune(lines[i])
		for j := 1; j < len(chars); j += 4 {
			if chars[j] != ' ' {
				stacks[(j-1)/4].push(chars[j])
			}
		}

		i--
	}

	moves := make([][3]int, len(lines)-begin)

	for j := begin; j < len(lines); j++ {
		words := strings.Fields(lines[j])
		count, _ := strconv.Atoi(words[1])
		index0, _ := strconv.Atoi(words[3])
		index1, _ := strconv.Atoi(words[5])
		moves[j-begin] = [3]int{count, index0 - 1, index1 - 1}
	}

	return InputType{
		stacks,
		moves,
	}
}

func part1(input InputType) RetType {

	for _, move := range input.moves {
		for i := 0; i < move[0]; i++ {
			input.stacks[move[2]].push(input.stacks[move[1]].pop())
		}

		// -----------------------------
		// uncomment for visualisation !
		// -----------------------------
		// fmt.Print("\n")
		// for _, v := range input.stacks {
		// 	v.print()
		// }
	}

	s := ""

	for _, v := range input.stacks {
		s += string(v.pop())
	}

	return s
}

func part2(input InputType) RetType {
	for _, move := range input.moves {
		temp_stack := Stack{
			value: 0,
			next:  nil,
		}

		for i := 0; i < move[0]; i++ {
			temp_stack.push(input.stacks[move[1]].pop())
		}

		for i := 0; i < move[0]; i++ {
			input.stacks[move[2]].push(temp_stack.pop())
		}

		// -----------------------------
		// uncomment for visualisation !
		// -----------------------------
		// fmt.Print("\n")
		// for _, v := range input.stacks {
		// 	v.print()
		// }
	}

	s := ""

	for _, v := range input.stacks {
		s += string(v.pop())
	}

	return s
}

package main

import "strconv"

type InputType []intFIFO
type RetType = int

type intFIFO struct {
	els        []int
	insert_ptr int
	pop_ptr    int
	c          int
}

func (f *intFIFO) insert(x int) {
	f.els[f.insert_ptr] = x
	f.insert_ptr = (f.insert_ptr + 1) % len(f.els)
	f.c++
}

func (f *intFIFO) pop() int {
	temp := f.els[f.pop_ptr]
	f.pop_ptr = (f.pop_ptr + 1) % len(f.els)
	f.c--
	return temp
}

func (f *intFIFO) count() int {
	return f.c
}

func (f *intFIFO) is_empty() int {
	return f.c
}

func make_intFIFO(capacity int) intFIFO {
	return intFIFO{
		make([]int, capacity),
		0,
		0,
		0,
	}
}

func fifo_from_list(list []int, capacity int) intFIFO {
	f := make_intFIFO(capacity)

	for _, v := range list {
		f.insert(v)
	}

	return f
}

func format(lines []string) InputType {
	nums := make([][]int, 0)
	current_num := make([]int, 0)

	for i := 1; i < len(lines); i++ {
		x, err := strconv.Atoi(lines[i])
		if err != nil {
			nums = append(nums, current_num)
			current_num = make([]int, 0)
		} else {
			current_num = append(current_num, x)
		}
	}

	nums = append(nums, current_num)

	fifos := make([]intFIFO, 2)
	n := len(nums[0]) + len(nums[1])
	fifos[0] = fifo_from_list(nums[0], n)
	fifos[1] = fifo_from_list(nums[1], n)

	return fifos
}

func (p *intFIFO) score() int {
	s := 0

	for p.count() != 0 {
		s += p.count() * p.pop()
	}

	return s
}

func (p *intFIFO) clone() intFIFO {
	f := make_intFIFO(len(p.els))

	for p.count() != 0 {
		f.insert(p.pop())
	}

	return f
}

func part1(input InputType) RetType {
	p0, p1 := input[0], input[1]
	n := p0.count() + p1.count()

	for p0.count() != n && p1.count() != n {
		c0 := p0.pop()
		c1 := p1.pop()

		if c0 > c1 {
			p0.insert(c0)
			p0.insert(c1)
		} else {
			p1.insert(c1)
			p1.insert(c0)
		}
	}

	if p0.count() == n {
		return p0.score()
	} else {
		return p1.score()
	}

}

func equal_intFIFO(f0 intFIFO, f1 intFIFO) bool {
	if f0.count() != f1.count() {
		return false
	}

	for f0.count() > 0 {
		if f0.pop() != f0.pop() {
			return false
		}
	}

	return true
}

type intFIFOSet map[int][]intFIFO

func (s *intFIFOSet) add(f intFIFO) {
	hash := f.score()

	l, ok := (*s)[hash]

	if !ok {
		(*s)[hash] = []intFIFO{f}
	} else {
		(*s)[hash] = append(l, f)
	}
}

func (s *intFIFOSet) has(f intFIFO) bool {
	hash := f.score()

	l, ok := (*s)[hash]

	if !ok {
		return false
	} else {
		for _, f1 := range l {
			if equal_intFIFO(f, f1) {
				return true
			}
		}

		return false
	}
}

func recursive_game(p0 intFIFO, p1 intFIFO, seen0 intFIFOSet, seen1 intFIFOSet) (int, intFIFO, intFIFO) {
	if seen0.has(p0) {
		return 0, p0, p1
	} else if seen1.has(p1) {
		return 1, p1, p1
	}

	c0 := p0.pop()
	c1 := p1.pop()

	if c0 <= p0.count() && c1 <= p1.count() {
		winner, new_p0, new_p1 := recursive_game(p0.clone(), p1.clone(), make(intFIFOSet), make(intFIFOSet))
	}

	return -1, p0, p1
}

func part2(input InputType) RetType {
	return 0
}

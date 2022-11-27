package main

import (
	"fmt"
	"sort"
	"strings"
)

type InputType = []Line
type RetType = int

type StringSet map[string]bool

func (s *StringSet) has(str string) bool {
	_, ok := (*s)[str]
	return ok
}

func (s *StringSet) add(str string) {
	(*s)[str] = true
}

func (s *StringSet) remove(str string) {
	delete((*s), str)
}

func (s0 *StringSet) union(s1 *StringSet) StringSet {
	union := make(StringSet)
	for k := range *s0 {
		union[k] = true
	}
	for k := range *s1 {
		union[k] = true
	}

	return union
}

func (s0 *StringSet) intersection(s1 *StringSet) StringSet {
	if len(*s0) > len(*s1) {
		s1, s0 = s0, s1
	}

	inter := make(StringSet)
	for k := range *s0 {
		if (*s1)[k] {
			inter[k] = true
		}
	}

	return inter
}

func (s *StringSet) get_any() string {
	for k := range *s {
		return k
	}

	panic("empty set")
}

func string_set_from(l []string) StringSet {
	s := make(StringSet)

	for _, v := range l {
		s.add(v)
	}

	return s
}

type Line struct {
	ingredients []string
	allergens   []string
}

func get_line(l string) Line {
	temp := strings.Split(l, " (")
	allergens := strings.Split(temp[1][9:len(temp[1])-1], ", ")
	ingredients := strings.Fields(temp[0])

	return Line{
		ingredients,
		allergens,
	}
}

func format(lines []string) InputType {
	var l = make([]Line, len(lines))

	for i := 0; i < len(lines); i++ {
		l[i] = get_line(lines[i])
	}

	return l
}

func find_assocs_and_occs(input InputType) (map[string]string, map[string]int) {
	var possible_ingredients = make(map[string]StringSet)
	var occurences = make(map[string]int)

	for _, line := range input {
		occ := string_set_from(line.ingredients)
		for _, allergen := range line.allergens {
			v, ok := possible_ingredients[allergen]
			if !ok {
				possible_ingredients[allergen] = occ
			} else {
				possible_ingredients[allergen] = v.intersection(&occ)
			}
		}

		for _, ing := range line.ingredients {
			k, ok := occurences[ing]
			if !ok {
				occurences[ing] = 1
			} else {
				occurences[ing] = k + 1
			}
		}
	}

	var associations = make(map[string]string)

	for {
		has_found_one_guilty := false
		for allergen, ings := range possible_ingredients {
			if len(ings) == 1 {
				guilty_ing := ings.get_any()
				associations[guilty_ing] = allergen

				for other_allergen := range possible_ingredients {
					delete(possible_ingredients[other_allergen], guilty_ing)
				}

				has_found_one_guilty = true
				break
			}
		}

		if !has_found_one_guilty {
			break
		}
	}

	return associations, occurences
}

func part1(input InputType) RetType {
	associations, occurences := find_assocs_and_occs(input)

	count := 0

	for ing, k := range occurences {
		_, ok := associations[ing]

		if !ok {
			count += k
		}
	}

	return count
}

func part2(input InputType) RetType {
	associations, _ := find_assocs_and_occs(input)
	associations_reversed := make(map[string]string)

	allergens := make([]string, len(associations))

	i := 0
	for ing, all := range associations {
		allergens[i] = all
		associations_reversed[all] = ing
		i++
	}

	sort.Strings(allergens)

	guilty_ings := make([]string, len(associations))
	for i := 0; i < len(allergens); i++ {
		guilty_ings[i] = associations_reversed[allergens[i]]
	}

	fmt.Println(strings.Join(guilty_ings, ","))

	return 0
}

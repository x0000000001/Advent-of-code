package main

import (
	"fmt"
	"strconv"
	"strings"
)

const (
	// https://stackoverflow.com/a/6878625/13123535
	MaxUint = ^uint(0)
	MaxInt  = int(MaxUint >> 1)
)

type InputType = *FileTree
type RetType = int

type FileTree struct {
	is_dir   bool
	size     int
	children map[string]*FileTree
	parent   *FileTree
}

func (t *FileTree) source_dir() *FileTree {
	if t.parent == nil {
		return t
	} else {
		return t.parent.source_dir()
	}
}

func (t *FileTree) print(indent int) {
	for name, c := range t.children {
		fmt.Printf("%s%s\n", strings.Repeat("   ", indent), name)
		c.print(indent + 1)
	}
}

func format(lines []string) InputType {
	current_dir := &FileTree{
		is_dir:   true,
		size:     0,
		children: make(map[string]*FileTree),
		parent:   nil,
	}

	for _, l := range lines {
		words := strings.Fields(l)

		if words[0] == "$" && words[1] == "cd" {
			// cd
			if words[2] == ".." {
				current_dir = current_dir.parent
			} else {
				if words[2] == "/" {
					// root
					current_dir = current_dir.source_dir()
				} else {
					//child
					child, ok := current_dir.children[words[2]]

					if !ok {
						panic("unknown child")
					}

					current_dir = child
				}

			}
		} else if words[0] != "$" {
			// ls result
			var new_child FileTree

			if words[0] == "dir" {
				new_child = FileTree{
					size:     0,
					is_dir:   true,
					children: make(map[string]*FileTree),
					parent:   current_dir,
				}
			} else {

				size, ok := strconv.Atoi(words[0])

				if ok != nil {
					panic(ok)
				}

				new_child = FileTree{
					size:     size,
					is_dir:   false,
					children: make(map[string]*FileTree),
					parent:   current_dir,
				}

				current_dir.children[words[1]] = &new_child
			}

			current_dir.children[words[1]] = &new_child
		}
	}

	return current_dir.source_dir()
}

// returns is_at_most_100000, size_of_this_folder, sum_of_sizes_with_children
func folder_sizes(t *FileTree) (bool, int, int) {
	if !t.is_dir {
		return (t.size <= 100000), t.size, 0
	}

	this_dir_size := 0
	children_sums := 0
	is_at_most_100000 := true

	for _, c := range t.children {
		ok, children_size, sub_children_size := folder_sizes(c)

		if !ok {
			is_at_most_100000 = false
		}

		this_dir_size += children_size
		children_sums += sub_children_size
	}

	if this_dir_size > 100000 {
		is_at_most_100000 = false
	}

	if is_at_most_100000 {
		children_sums += this_dir_size
	}

	return is_at_most_100000, this_dir_size, children_sums
}

func part1(input InputType) RetType {
	_, _, res := folder_sizes(input)
	return res
}

func (t *FileTree) space() int {
	if !t.is_dir {
		return t.size
	}

	sum := 0

	for _, c := range t.children {
		sum += c.space()
	}

	return sum
}

// returns (folder_size, smallest_valid_delete_size_in_children)
func smallest_delete(t *FileTree, required_space int) (int, int) {
	if !t.is_dir {
		return t.size, MaxInt
	}

	minimum_delete := MaxInt
	this_dir_size := 0

	for _, c := range t.children {
		children_size, min_in_children := smallest_delete(c, required_space)
		this_dir_size += children_size
		if min_in_children < minimum_delete {
			minimum_delete = min_in_children
		}
	}

	if this_dir_size >= required_space && this_dir_size < minimum_delete {
		minimum_delete = this_dir_size
	}

	return this_dir_size, minimum_delete
}

func part2(input InputType) RetType {
	required_space := 30000000 - (70000000 - input.space())
	_, min := smallest_delete(input, required_space)
	return min
}

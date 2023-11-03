package main

type InputType [][]uint8
type RetType = int

func format(lines []string) InputType {
	grid := make([][]uint8, len(lines))

	for i := 0; i < len(lines); i++ {
		chars := []rune(lines[i])
		grid[i] = make([]uint8, len(chars))

		for j, c := range chars {
			grid[i][j] = uint8(c - '0')
		}
	}

	return grid
}

func part1(grid InputType) RetType {
	h, w := len(grid), len(grid[0])

	count := 0
	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			current := grid[i][j]
			visible := true
			for k := i - 1; k >= 0; k-- {
				if grid[k][j] >= current {
					visible = false
					break
				}
			}
			if visible {
				count++
				continue
			}
			visible = true
			for k := i + 1; k < h; k++ {
				if grid[k][j] >= current {
					visible = false
					break
				}
			}
			if visible {
				count++
				continue
			}
			visible = true
			for k := j - 1; k >= 0; k-- {
				if grid[i][k] >= current {
					visible = false
					break
				}
			}
			if visible {
				count++
				continue
			}
			visible = true
			for k := j + 1; k < w; k++ {
				if grid[i][k] >= current {
					visible = false
					break
				}
			}
			if visible {
				count++
				continue
			}
		}
	}

	return count
}

func part2(grid InputType) RetType {
	h, w := len(grid), len(grid[0])

	max := 0
	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			current := grid[i][j]
			score := 1
			visible := 0
			for k := i - 1; k >= 0; k-- {
				visible++
				if grid[k][j] >= current {
					break
				}
			}
			score *= visible
			visible = 0
			for k := i + 1; k < h; k++ {
				visible++
				if grid[k][j] >= current {
					break
				}
			}
			score *= visible
			visible = 0
			for k := j - 1; k >= 0; k-- {
				visible++
				if grid[i][k] >= current {
					break
				}
			}
			score *= visible
			visible = 0
			for k := j + 1; k < w; k++ {
				visible++
				if grid[i][k] >= current {
					break
				}
			}
			score *= visible

			if score > max {
				max = score
			}
		}
	}

	return max
}

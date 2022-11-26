package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Grid [][]bool

type Tile struct {
	g000 Grid
	g001 Grid
	g010 Grid
	g011 Grid
	g100 Grid
	g101 Grid
	g110 Grid
	g111 Grid
}

type InputType = map[int]Grid
type RetType = int

func flip_hoz(g Grid) Grid {
	w := len(g)
	g1 := make([][]bool, w)

	for i := range g1 {
		g1[i] = make([]bool, w)
	}

	for i := 0; i < w; i++ {
		for j := 0; j < w; j++ {
			g1[i][j] = g[w-i-1][j]
		}
	}

	return g1
}

func flip_vert(g Grid) Grid {
	w := len(g)
	g1 := make([][]bool, w)

	for i := range g1 {
		g1[i] = make([]bool, w)
	}

	for i := 0; i < w; i++ {
		for j := 0; j < w; j++ {
			g1[i][j] = g[i][w-j-1]
		}
	}

	return g1
}

func rotate_90(g Grid) Grid {
	w := len(g)
	g1 := make([][]bool, w)

	for i := range g1 {
		g1[i] = make([]bool, w)
	}

	for i := 0; i < w; i++ {
		for j := 0; j < w; j++ {
			g1[i][j] = g[j][w-i-1]
		}
	}

	return g1
}

// g1 relative to g0
func possible_rotations(g0 Grid, g1 Grid) []int {
	w := len(g0)
	var rots []int

	must_be_added := true
	for i := 0; i < w; i++ {
		if g0[0][i] != g1[w-1][i] {
			must_be_added = false
			break
		}
	}
	if must_be_added {
		rots = append(rots, 0)
	}

	must_be_added = true
	for i := 0; i < w; i++ {
		if g0[w-1][i] != g1[0][i] {
			must_be_added = false
			break
		}
	}
	if must_be_added {
		rots = append(rots, 2)
	}

	must_be_added = true
	for i := 0; i < w; i++ {
		if g0[i][w-1] != g1[i][0] {
			must_be_added = false
			break
		}
	}
	if must_be_added {
		rots = append(rots, 1)
	}

	must_be_added = true
	for i := 0; i < w; i++ {
		if g0[i][0] != g1[i][w-1] {
			must_be_added = false
			break
		}
	}
	if must_be_added {
		rots = append(rots, 3)
	}

	return rots
}

func (g *Grid) get_tile() Tile {
	return Tile{
		*g,
		flip_hoz(*g),
		flip_vert(*g),
		flip_hoz(flip_vert(*g)),
		rotate_90(*g),
		rotate_90(flip_hoz(*g)),
		rotate_90(flip_vert(*g)),
		rotate_90(flip_hoz(flip_vert(*g))),
	}
}

// we don't care about rotation since
// flipping vert then hoz is equivalent

func line_to_bools(s string) (b []bool) {
	for _, c := range s {
		b = append(b, c == '#')
	}

	return
}

func format(lines []string) InputType {
	grids := make(map[int]Grid)
	current_grid := make([][]bool, 0)
	current_grid_id := 0
	i := 0

	for i < len(lines) {
		words := strings.Fields(lines[i])
		if len(words) > 1 {
			if len(current_grid) > 0 {
				grids[current_grid_id] = current_grid
			}

			current_grid = make([][]bool, 0)
			current_grid_id, _ = strconv.Atoi(words[1][:len(words[1])-1])
			i++
			continue
		}

		current_grid = append(current_grid, line_to_bools(lines[i]))
		i++
	}

	if len(current_grid) > 0 {
		grids[current_grid_id] = current_grid
	}

	return grids
}

type Puzzle struct {
	available_grids []int
	grid_ids        [][]int
	orientations    [][]int
	grids           *map[int]Tile
	x               int
	y               int
	w               int
}

func (p *Puzzle) get_grid(x int, y int) Grid {
	switch p.orientations[y][x] {
	case 0:
		return (*p.grids)[p.grid_ids[y][x]].g000
	case 1:
		return (*p.grids)[p.grid_ids[y][x]].g001
	case 10:
		return (*p.grids)[p.grid_ids[y][x]].g010
	case 11:
		return (*p.grids)[p.grid_ids[y][x]].g011
	case 100:
		return (*p.grids)[p.grid_ids[y][x]].g100
	case 101:
		return (*p.grids)[p.grid_ids[y][x]].g101
	case 110:
		return (*p.grids)[p.grid_ids[y][x]].g110
	case 111:
		return (*p.grids)[p.grid_ids[y][x]].g111
	default:
		panic("invalid orientation")
	}
}

func add_value_to_grid(value int, grid_ids *[][]int, x int, y int, w int) [][]int {
	duplicate := make([][]int, w)
	for i := range *grid_ids {
		duplicate[i] = make([]int, w)
		copy(duplicate[i], (*grid_ids)[i])
	}

	duplicate[y][x] = value

	return duplicate
}

func contains(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

// h and w correspond to each grid height and length
func solve_puzzle(p Puzzle) ([][]int, Puzzle, bool) {
	n := len(p.grid_ids)
	if p.y == n {
		return p.grid_ids, p, true
	}

	require_top, require_left := false, false

	if p.y > 0 {
		require_top = true
	}
	if p.x > 0 {
		require_left = true
	}

	for i, new_tile_id := range p.available_grids {
		new_available_grids := make([]int, len(p.available_grids))
		copy(new_available_grids, p.available_grids)
		// https://stackoverflow.com/a/37335777/13123535
		new_available_grids[i] = new_available_grids[len(new_available_grids)-1]
		new_available_grids = new_available_grids[:len(new_available_grids)-1]
		new_tile, _ := (*p.grids)[new_tile_id]
		new_grid_ids := add_value_to_grid(new_tile_id, &p.grid_ids, p.x, p.y, n)
		possibilities := [8]Grid{
			new_tile.g000, new_tile.g001, new_tile.g010, new_tile.g011,
			new_tile.g100, new_tile.g101, new_tile.g110, new_tile.g111,
		}

		rotations := [8]int{
			0, 1, 10, 11, 100, 101, 110, 111,
		}
		newx := p.x + 1
		newy := p.y
		if newx == n {
			newx = 0
			newy++
		}

		for rot_id, g := range possibilities {
			if require_top {
				if !contains(possible_rotations(p.get_grid(p.x, p.y-1), g), 2) {
					continue
				}
			}

			if require_left {
				if !contains(possible_rotations(p.get_grid(p.x-1, p.y), g), 1) {
					continue
				}
			}

			// grid can be placed
			new_orientations := add_value_to_grid(rotations[rot_id], &p.orientations, p.x, p.y, n)
			result, new_p, ok := solve_puzzle(Puzzle{
				available_grids: new_available_grids,
				grid_ids:        new_grid_ids,
				orientations:    new_orientations,
				grids:           p.grids,
				x:               newx,
				y:               newy,
				w:               p.w,
			})

			if ok {
				return result, new_p, true
			}
		}
	}

	return [][]int{}, Puzzle{}, false
}

func (p *Puzzle) print() {
	fmt.Println()
	n := len(p.grid_ids)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			id := p.grid_ids[i][j]

			if id != -1 {
				fmt.Println(id, i, j)
				g := p.get_grid(j, i)
				for k := 0; k < p.w; k++ {
					for l := 0; l < p.w; l++ {
						if g[k][l] {
							fmt.Print("#")
						} else {
							fmt.Print(".")
						}
					}
					fmt.Println()
				}
			}
		}
	}
}

func gridInt2d(w int, h int, default_value int) [][]int {
	g := make([][]int, h)

	for i := range g {
		g[i] = make([]int, w)
		for j := 0; j < w; j++ {
			g[i][j] = default_value
		}
	}

	return g
}

type Point struct {
	x int
	y int
}

func get_solved_puzzle(tiles map[int]Tile) ([][]int, Puzzle) {
	n := int(math.Sqrt(float64(len(tiles))))
	var w int
	for _, v := range tiles {
		w = len(v.g000)
		break
	}
	var available_grids []int
	for k := range tiles {
		available_grids = append(available_grids, k)
	}

	p := Puzzle{
		available_grids: available_grids,
		grid_ids:        gridInt2d(n, n, -1),
		orientations:    gridInt2d(n, n, -1),
		grids:           &tiles,
		x:               0,
		y:               0,
		w:               w,
	}

	grid, p, ok := solve_puzzle(p)

	if !ok {
		panic("no arrangement")
	}

	return grid, p
}

func part1(input InputType) RetType {
	n := int(math.Sqrt(float64(len(input))))

	var tiles map[int]Tile = make(map[int]Tile)

	for k, v := range input {
		tiles[k] = v.get_tile()
	}

	grid, _ := get_solved_puzzle(tiles)

	return grid[0][0] * grid[n-1][0] * grid[0][n-1] * grid[n-1][n-1]
}

func get_image(p Puzzle, grid [][]int, n int) [][]bool {

	width := (p.w - 2) * n

	image := make([][]bool, width)

	for i := 0; i < width; i++ {
		image[i] = make([]bool, width)
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			g := p.get_grid(j, i)
			for k := 1; k < p.w-1; k++ {
				for l := 1; l < p.w-1; l++ {
					image[i*(p.w-2)+k-1][j*(p.w-2)+l-1] = g[k][l]
				}
			}
		}
	}

	return image
}

type Pos struct {
	x int
	y int
}

var monster_scheme = []Pos{{0, 1}, {1, 2}, {4, 2}, {5, 1}, {6, 1}, {7, 2}, {10, 2}, {11, 1}, {12, 1}, {13, 2}, {16, 2}, {17, 1}, {18, 1}, {18, 0}, {19, 1}}
var monster_width = 20
var monster_height = 3

func monster_count(image [][]bool) int {
	w := len(image)
	count := 0
	for i := 0; i < w-monster_height; i++ {
		for j := 0; j < w-monster_width; j++ {
			is_monster := true
			for _, v := range monster_scheme {
				if !image[i+v.y][j+v.x] {
					is_monster = false
					break
				}
			}
			if is_monster {
				count++
			}
		}
	}

	return count
}

func part2(input InputType) RetType {
	n := int(math.Sqrt(float64(len(input))))

	var tiles map[int]Tile = make(map[int]Tile)

	for k, v := range input {
		tiles[k] = v.get_tile()
	}

	grid, puzzle := get_solved_puzzle(tiles)

	image := get_image(puzzle, grid, n)
	w := len(image)

	depths_count := 0
	for i := 0; i < w; i++ {
		for j := 0; j < w; j++ {
			if image[i][j] {
				depths_count++
			}
		}
	}

	possible_images := [][][]bool{
		image,
		flip_hoz(image),
		flip_vert(image),
		flip_hoz(flip_vert(image)),
		rotate_90(image),
		rotate_90(flip_hoz(image)),
		rotate_90(flip_vert(image)),
		rotate_90(flip_hoz(flip_vert(image))),
	}

	for _, v := range possible_images {
		c := monster_count(v)
		if c != 0 {
			return depths_count - c*len(monster_scheme)
		}
	}

	return 0
}

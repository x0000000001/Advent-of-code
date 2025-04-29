use std::{collections::HashSet, fs};

type InputType = Vec<Vec<usize>>;
type Pos = (usize, usize);

fn neighbors((i, j): Pos, w: usize) -> Vec<Pos> {
    let mut neighs = vec![];

    for (xi, xj) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
        if i + xi == 0 || i + xi > w || j + xj == 0 || j + xj > w {
            continue;
        }

        neighs.push((i + xi - 1, j + xj - 1));
    }

    return neighs;
}

fn explore(
    grid: &InputType,
    pos: Pos,
    final_positions: &mut HashSet<Pos>,
    distincts_count: &mut usize,
) {
    let value = grid[pos.0][pos.1];
    if value == 9 {
        *distincts_count += 1;
        final_positions.insert(pos.clone());
    }

    neighbors(pos, grid.len())
        .into_iter()
        .filter(|p| grid[p.0][p.1] == value + 1)
        .for_each(|p| explore(grid, p, final_positions, distincts_count));
}

pub fn solve(grid: InputType) {
    let w = grid.len();
    let mut sum = 0;
    let mut sum_distincts = 0;

    for i in 0..w {
        for j in 0..w {
            if grid[i][j] != 0 {
                continue;
            }

            let mut s = HashSet::new();
            let mut distincts_count = 0;
            explore(&grid, (i, j), &mut s, &mut distincts_count);
            sum += s.len();
            sum_distincts += distincts_count;
        }
    }

    println!("{} {}", sum, sum_distincts);
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|str| {
            str.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn main() {
    for file in ["data/input_example.txt", "data/input.txt"] {
        // for file in ["data/input_example.txt"] {
        let s = fs::read_to_string(file).unwrap();
        solve(parse(s.clone()));
    }
}

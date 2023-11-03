use crate::Solution;

// (used, capacity)
type InputType = Vec<Vec<(u64, u64)>>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);
    let mut count = 0;

    let nodes = input
        .into_iter()
        .flat_map(|l| l)
        .collect::<Vec<(u64, u64)>>();

    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }

            if nodes[i].0 != 0 && (nodes[j].1 - nodes[j].0) >= nodes[i].0 {
                count += 1;
            }
        }
    }

    Solution::from(count)
}

#[allow(dead_code)]
fn print_grid(grid: &InputType) {
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            let (used, cap) = grid[x][y];
            if used == 0 {
                print!("_ ");
            } else if cap > 100 {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

/// Data is exactly as described in the example. \
/// There is a horizontal wall, so you first have to bring empty to x = 0. \
/// (use print_grid)
pub fn part2(s: String) -> Solution {
    let grid = parse(s);

    let mut empty = (0, 0);
    let (w, _) = (grid.len(), grid[0].len());

    'main_loop: for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            let (used, _) = grid[x][y];
            if used == 0 {
                empty = (x, y);
                break 'main_loop;
            }
        }
    }

    let mut moves = 0;
    // go back to x = 0
    moves += empty.0;
    // go back to y = 0
    moves += empty.1;
    // go to x = w-2
    moves += w - 2;
    // bring goal to x = 1
    moves += 5 * (w - 2);
    // bring goal to x = 0
    moves += 1;

    Solution::from(moves as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    // determining W and H
    let words = input
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let w1 = words[0].split("-").collect::<Vec<&str>>();
    let w = w1[1].replace("x", "").parse::<usize>().unwrap() + 1;
    let h = w1[2].replace("y", "").parse::<usize>().unwrap() + 1;

    let mut res: InputType = vec![vec![(0, 0); h]; w];

    for l in input {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        if &w[0][0..4] != "/dev" {
            continue;
        }

        let w1 = w[0].split("-").collect::<Vec<&str>>();
        let x = w1[1].replace("x", "").parse::<usize>().unwrap();
        let y = w1[2].replace("y", "").parse::<usize>().unwrap();
        let used = w[2].replace("T", "").parse::<u64>().unwrap();
        let capacity = w[1].replace("T", "").parse::<u64>().unwrap();

        res[x][y] = (used, capacity);
    }

    res
}

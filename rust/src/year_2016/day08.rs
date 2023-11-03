use crate::Solution;

type InputType = Vec<(i64, usize, usize)>;

fn get_grid(g: &Vec<Vec<bool>>) -> String {
    let mut s = String::new();

    for j in 0..6 {
        for i in 0..50 {
            s += if g[i][j] { "#" } else { "." };
        }
        s += "\n";
    }

    s
}

fn solve(input: InputType) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 6]; 50];

    for (order, n0, mut n1) in input {
        match order {
            0 => {
                for i in 0..n0 {
                    for j in 0..n1 {
                        grid[i][j] = true;
                    }
                }
            }
            1 => {
                let temp: Vec<bool> = (0..50).map(|i| grid[i][n0]).collect();
                n1 = n1 % 50;
                for i in 0..50 {
                    let index = if n1 > i { 50 - n1 + i } else { i - n1 };
                    grid[i][n0] = temp[index];
                }
            }
            _ => {
                let temp: Vec<bool> = grid[n0].clone();
                n1 = n1 % 6;
                for j in 0..6 {
                    let index = if n1 > j { 6 - n1 + j } else { j - n1 };
                    grid[n0][j] = temp[index];
                }
            }
        }
    }

    grid
}

// 0 = rect, 1 = row, 2 = column
pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        solve(input)
            .into_iter()
            .flat_map(|l| l.into_iter().map(|c| if c { 1 } else { 0 }))
            .sum::<i64>(),
    )
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(get_grid(&mut solve(input)))
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let w: Vec<&str> = l.split_whitespace().collect();
        if w[0] == "rect" {
            let w1: Vec<&str> = w[1].split("x").collect();
            res.push((0, w1[0].parse().unwrap(), w1[1].parse().unwrap()));
        } else {
            let begin: usize = w[2].split("=").last().unwrap().parse().unwrap();
            let amplitude: usize = w.last().unwrap().parse().unwrap();

            if w[1] == "row" {
                res.push((1, begin, amplitude))
            } else {
                res.push((2, begin, amplitude))
            }
        }
    }

    res
}

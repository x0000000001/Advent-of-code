use std::collections::HashSet;

use crate::Solution;

type InputType = HashSet<Position>;

type Position = (i64, i64);
type Depth = i64;

fn iter_bugs(bugs: &HashSet<Position>) -> HashSet<Position> {
    let mut new_bugs = HashSet::new();

    for x in 0..5 {
        for y in 0..5 {
            let count = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|p| bugs.contains(p))
                .count();

            if bugs.contains(&(x, y)) {
                if count == 1 {
                    new_bugs.insert((x, y));
                }
            } else {
                if count == 1 || count == 2 {
                    new_bugs.insert((x, y));
                }
            }
        }
    }

    new_bugs
}

fn hash_bugs(bugs: &HashSet<Position>) -> usize {
    let mut res = 0;

    for x in (0..5).rev() {
        for y in (0..5).rev() {
            res *= 2;

            if bugs.contains(&(x, y)) {
                res += 1;
            }
        }
    }

    res
}

#[allow(dead_code)]
fn print_bugs(bugs: &HashSet<Position>, h: usize, w: usize) {
    for i in 0..h as i64 {
        for j in 0..w as i64 {
            print!("{}", if bugs.contains(&(i, j)) { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

pub fn part1(s: String) -> Solution {
    let mut bugs = parse(s);

    let mut seen = HashSet::new();

    loop {
        let hash = hash_bugs(&bugs);
        if seen.contains(&hash) {
            return Solution::from(hash as i64);
        }

        seen.insert(hash);
        bugs = iter_bugs(&bugs);
    }
}

fn iter_bugs2(bugs: &HashSet<(Position, Depth)>) -> HashSet<(Position, Depth)> {
    let min_depth = bugs.iter().map(|(_, d)| *d).min().unwrap();
    let max_depth = bugs.iter().map(|(_, d)| *d).max().unwrap();
    let mut new_bugs = HashSet::new();

    for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 {
                continue;
            }

            for depth in (min_depth - 1)..(max_depth + 2) {
                let pos = ((x, y), depth);
                let count = get_neighbors(pos)
                    .into_iter()
                    .filter(|p| bugs.contains(p))
                    .count();

                if bugs.contains(&pos) {
                    if count == 1 {
                        new_bugs.insert(pos);
                    }
                } else {
                    if count == 1 || count == 2 {
                        new_bugs.insert(pos);
                    }
                }
            }
        }
    }

    new_bugs
}

fn get_neighbors(((x, y), d): ((i64, i64), i64)) -> Vec<(Position, Depth)> {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .flat_map(|(i, j)| {
            if i == 2 && j == 2 {
                match (x, y) {
                    (1, 2) => (0..5).map(|jp| ((0, jp), d + 1)).collect(),
                    (3, 2) => (0..5).map(|jp| ((4, jp), d + 1)).collect(),
                    (2, 1) => (0..5).map(|ip| ((ip, 0), d + 1)).collect(),
                    (2, 3) => (0..5).map(|ip| ((ip, 4), d + 1)).collect(),
                    _ => panic!(),
                }
            } else if i < 0 {
                Vec::from([((1, 2), d - 1)])
            } else if i > 4 {
                Vec::from([((3, 2), d - 1)])
            } else if j < 0 {
                Vec::from([((2, 1), d - 1)])
            } else if j > 4 {
                Vec::from([((2, 3), d - 1)])
            } else {
                Vec::from([((i, j), d)])
            }
        })
        .collect()
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut bugs = input.into_iter().map(|p| (p, 0)).collect();

    for _ in 0..200 {
        bugs = iter_bugs2(&bugs);
    }

    Solution::from(bugs.len() as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input
        .into_iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| (i as i64, j as i64))
                .collect::<Vec<Position>>()
        })
        .collect()
}

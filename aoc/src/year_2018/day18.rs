use std::collections::HashMap;

use crate::Solution;

type InputType = Collection;

pub type Collection = Vec<Vec<State>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Ground,
    Trees,
    Yard,
}

fn iter_collection(c: &Collection) -> Collection {
    let (h, w) = (c.len(), c[0].len());

    let mut new = vec![vec![State::Ground; w]; h];

    let count_neighbors = |x: usize, y: usize, s: State| -> usize {
        (-1..2)
            .flat_map(|i| (-1..2).map(move |j| (i, j)))
            .map(|(i, j)| (x as i64 + i, y as i64 + j))
            .filter(|&(i, j)| i >= 0 && i < h as i64 && j >= 0 && j < w as i64)
            .map(|(i, j)| (i as usize, j as usize))
            .filter(|&(i, j)| !(i == x && j == y) && c[i][j].eq(&s))
            .count()
    };

    for i in 0..h {
        for j in 0..w {
            new[i][j] = match c[i][j] {
                State::Ground => {
                    if count_neighbors(i, j, State::Trees) >= 3 {
                        State::Trees
                    } else {
                        State::Ground
                    }
                }
                State::Trees => {
                    if count_neighbors(i, j, State::Yard) >= 3 {
                        State::Yard
                    } else {
                        State::Trees
                    }
                }
                State::Yard => {
                    if count_neighbors(i, j, State::Yard) > 0
                        && count_neighbors(i, j, State::Trees) > 0
                    {
                        State::Yard
                    } else {
                        State::Ground
                    }
                }
            };
        }
    }

    new
}

#[allow(dead_code)]
fn print_collection(c: &Collection) {
    for i in 0..c.len() {
        for j in 0..c[i].len() {
            print!(
                "{}",
                match c[i][j] {
                    State::Ground => '.',
                    State::Trees => '|',
                    State::Yard => '#',
                }
            );
        }

        println!();
    }
    println!();
}

fn count_state(c: &Collection, state: State) -> usize {
    (0..c.len())
        .map(|i| {
            (0..c[0].len())
                .map(|j| c[i][j].eq(&state) as usize)
                .sum::<usize>()
        })
        .sum()
}

pub fn part1(s: String) -> Solution {
    let mut c = parse(s);

    for _ in 0..10 {
        // print_collection(&c);
        c = iter_collection(&c);
    }

    Solution::from((count_state(&c, State::Trees) * count_state(&c, State::Yard)) as i64)
}

pub fn part2(s: String) -> Solution {
    let mut c = parse(s);

    let mut seen = HashMap::new();
    let mut t = 0;

    while t < 1000000000 {
        if let Some(previous_t) = seen.get(&c) {
            let cycle = t - previous_t;
            let steps = (1000000000 - t) / cycle;
            if steps > 0 {
                t += steps * cycle;
                continue;
            }
        }

        seen.insert(c.clone(), t);
        c = iter_collection(&c);
        t += 1;
    }

    Solution::from((count_state(&c, State::Trees) * count_state(&c, State::Yard)) as i64)
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
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => State::Ground,
                    '|' => State::Trees,
                    '#' => State::Yard,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

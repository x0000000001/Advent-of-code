use std::collections::{HashMap, HashSet};

use crate::Solution;

type InputType = Vec<i128>;

struct Prg {
    i: i128,
    relative_base: i128,
    instrs: HashMap<usize, i128>,
    inputs: Vec<i128>,
    outputs: Vec<i128>,
}

#[derive(Clone, Copy)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Prg {
    fn eval_i(&self, i: i128, m: Mode) -> i128 {
        match m {
            Mode::Immediate => self.get(i),
            Mode::Position => self.get(self.get(i)),
            Mode::Relative => self.get(self.get(i) + self.relative_base),
        }
    }

    fn get(&self, i: i128) -> i128 {
        *self.instrs.get(&(i as usize)).or(Some(&0)).unwrap()
    }

    fn set(&mut self, i: i128, val: i128, mode: Mode) {
        match mode {
            Mode::Position | Mode::Immediate => {
                let entry = self.instrs.entry(i as usize).or_insert(0);
                *entry = val;
            }
            Mode::Relative => {
                let entry = self
                    .instrs
                    .entry((i + self.relative_base) as usize)
                    .or_insert(0);
                *entry = val;
            }
        }
    }
}

fn run_intcode(prg: &mut Prg) {
    while prg.i >= 0 && prg.i < prg.instrs.len() as i128 {
        let instr = prg.get(prg.i);
        let code = instr % 100;
        let modes_str = if instr > 100 {
            ((instr - code) / 100).to_string()
        } else {
            "".to_string()
        };
        let mut modes: Vec<Mode> = vec![Mode::Position; 3];
        let args = [prg.get(prg.i + 1), prg.get(prg.i + 2), prg.get(prg.i + 3)];

        for (i, c) in modes_str.chars().rev().enumerate() {
            modes[i] = match c {
                '0' => Mode::Position,
                '1' => Mode::Immediate,
                '2' => Mode::Relative,
                _ => panic!(),
            }
        }

        match code {
            99 => return,
            1 => {
                // add
                prg.set(
                    args[2],
                    prg.eval_i(prg.i + 1, modes[0]) + prg.eval_i(prg.i + 2, modes[1]),
                    modes[2],
                );
                prg.i += 4;
            }
            2 => {
                // mul
                prg.set(
                    args[2],
                    prg.eval_i(prg.i + 1, modes[0]) * prg.eval_i(prg.i + 2, modes[1]),
                    modes[2],
                );
                prg.i += 4;
            }
            3 => {
                // input
                if let Some(x) = prg.inputs.pop() {
                    prg.set(args[0], x, modes[0]);
                } else {
                    return;
                }
                prg.i += 2;
            }
            4 => {
                // output
                prg.outputs.push(prg.eval_i(prg.i + 1, modes[0]));
                prg.i += 2;
            }
            5 => {
                // jump if not 0
                if prg.eval_i(prg.i + 1, modes[0]) != 0 {
                    prg.i = prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            6 => {
                // jump if 0
                if prg.eval_i(prg.i + 1, modes[0]) == 0 {
                    prg.i = prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            7 => {
                // is less than
                if prg.eval_i(prg.i + 1, modes[0]) < prg.eval_i(prg.i + 2, modes[1]) {
                    prg.set(args[2], 1, modes[2]);
                } else {
                    prg.set(args[2], 0, modes[2]);
                }
                prg.i += 4;
            }
            8 => {
                // is equal
                if prg.eval_i(prg.i + 1, modes[0]) == prg.eval_i(prg.i + 2, modes[1]) {
                    prg.set(args[2], 1, modes[2]);
                } else {
                    prg.set(args[2], 0, modes[2]);
                }
                prg.i += 4;
            }
            // relative base
            9 => {
                prg.relative_base += prg.eval_i(prg.i + 1, modes[0]);
                prg.i += 2;
            }
            _ => panic!("wrong code : {code}"),
        }
    }
}

struct Map {
    grid: Vec<Vec<bool>>,
    start: (usize, usize),
    oxygen: (usize, usize),
}

fn get_map(input: InputType) -> Map {
    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();
    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: Vec::from([2]),
        outputs: Vec::new(),
    };

    let (mut x, mut y) = (0, 0);
    let mut walls: HashSet<(i64, i64)> = HashSet::new();

    let mut orientation = 1;
    let (mut goalx, mut goaly) = (0, 0);

    // WHY does this optimisation doesn't work ?
    // let mut seen_on_the_right = HashSet::new();
    // let mut is_following_wall = false;

    for _ in 0..5000 {
        // follow wall on the right
        while walls.contains(&next(x, y, orientation)) {
            orientation = match orientation {
                1 => 3,
                3 => 2,
                2 => 4,
                4 => 1,
                _ => panic!(),
            };
        }

        prg.inputs = Vec::from([orientation]);

        run_intcode(&mut prg);
        match prg.outputs.pop().unwrap() {
            0 => {
                // is_following_wall = true;
                walls.insert(next(x, y, orientation));
            }
            1 => {
                (x, y) = next(x, y, orientation);
                // if is_following_wall {
                //     let right_orientation = match orientation {
                //         1 => 4,
                //         4 => 2,
                //         2 => 3,
                //         3 => 1,
                //         _ => panic!(),
                //     };
                //     let grid_on_the_right = next(x, y, right_orientation);
                //     if seen_on_the_right.contains(&grid_on_the_right) {
                //         println!("{} {} {:?}", x, y, grid_on_the_right);
                //         break;
                //     } else {
                //         seen_on_the_right.insert(grid_on_the_right);
                //     }
                // }
                orientation = match orientation {
                    1 => 4,
                    4 => 2,
                    2 => 3,
                    3 => 1,
                    _ => panic!(),
                };
            }
            2 => {
                (x, y) = next(x, y, orientation);
                (goalx, goaly) = (x, y);
            }
            _ => panic!("unknown answer"),
        }
    }

    let (minx, maxx, miny, maxy) = walls.iter().fold(
        (i64::MAX, 0, i64::MAX, 0),
        |(minx, maxx, miny, maxy), (x, y)| (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y)),
    );

    let mut grid = vec![vec![false; (maxy - miny + 1) as usize]; (maxx - minx + 1) as usize];

    for (x, y) in walls {
        grid[(x - minx) as usize][(y - miny) as usize] = true;
    }

    Map {
        grid,
        start: (-minx as usize, -miny as usize),
        oxygen: ((goalx - minx) as usize, (goaly - miny) as usize),
    }
}

fn shortest_path(map: Map) -> Option<usize> {
    let mut scores = vec![vec![usize::MAX; map.grid.len()]; map.grid[0].len()];
    let mut queue = vec![];
    queue.push((map.start, 0));

    while !queue.is_empty() {
        queue.sort_by(|(_, score0), (_, score1)| score1.cmp(score0));
        let ((currentx, currenty), current_score) = queue.pop().unwrap();
        if (currentx, currenty) == map.oxygen {
            return Some(current_score);
        }

        for candidate_pos in [
            (currentx as i64 + 1, currenty as i64),
            (currentx as i64 - 1, currenty as i64),
            (currentx as i64, currenty as i64 + 1),
            (currentx as i64, currenty as i64 - 1),
        ] {
            if candidate_pos.0 < 0
                || candidate_pos.0 >= map.grid.len() as i64
                || candidate_pos.1 < 0
                || candidate_pos.1 >= map.grid[0].len() as i64
            {
                continue;
            }

            let (indexx, indexy) = (candidate_pos.0 as usize, candidate_pos.1 as usize);

            if map.grid[indexx][indexy] {
                continue;
            }

            let candidate_score = scores[indexx][indexy];

            if current_score + 1 < candidate_score {
                scores[indexx][indexy] = current_score + 1;
                queue.push(((indexx, indexy), current_score + 1));
            }
        }
    }

    None
}

fn next(x: i64, y: i64, orientation: i128) -> (i64, i64) {
    match orientation {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!(),
    }
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(shortest_path(get_map(input)).unwrap() as i64)
}

fn oxygen_time_path(map: Map) -> usize {
    let mut scores = vec![vec![usize::MAX; map.grid.len()]; map.grid[0].len()];
    let mut queue = vec![];
    queue.push((map.oxygen, 0));
    let mut max = 0;

    while !queue.is_empty() {
        queue.sort_by(|(_, score0), (_, score1)| score1.cmp(score0));
        let ((currentx, currenty), current_score) = queue.pop().unwrap();
        if current_score > max {
            max = current_score;
        }

        for candidate_pos in [
            (currentx as i64 + 1, currenty as i64),
            (currentx as i64 - 1, currenty as i64),
            (currentx as i64, currenty as i64 + 1),
            (currentx as i64, currenty as i64 - 1),
        ] {
            if candidate_pos.0 < 0
                || candidate_pos.0 >= map.grid.len() as i64
                || candidate_pos.1 < 0
                || candidate_pos.1 >= map.grid[0].len() as i64
            {
                continue;
            }

            let (indexx, indexy) = (candidate_pos.0 as usize, candidate_pos.1 as usize);

            if map.grid[indexx][indexy] {
                continue;
            }

            let candidate_score = scores[indexx][indexy];

            if current_score + 1 < candidate_score {
                scores[indexx][indexy] = current_score + 1;
                queue.push(((indexx, indexy), current_score + 1));
            }
        }
    }

    max
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(oxygen_time_path(get_map(input)) as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

use itertools::Itertools;
use std::{collections::HashMap, fs};

pub type InputType = Vec<i128>;

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

fn sum_inter(grid: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1000 {
                continue;
            }

            if i > 0
                && i < grid.len() - 1
                && j > 0
                && j < grid[0].len() - 1
                && grid[i - 1][j] != 1000
                && grid[i + 1][j] != 1000
                && grid[i][j - 1] != 1000
                && grid[i][j + 1] != 1000
            {
                sum += i * j;
            }
        }
    }

    sum
}

fn grid_from_prg(prg: &Prg) -> Vec<Vec<usize>> {
    let s = prg.outputs.iter().fold("".to_string(), |acc, i| {
        format!("{}{}", acc, char::from(*i as u8))
    });

    s.split("\n")
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => 2000,
                    '.' => 1000,
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => panic!(),
                })
                .collect::<Vec<usize>>()
        })
        .filter(|l| l.len() != 0)
        .collect()
}

pub fn result_1(input: InputType) -> i64 {
    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    run_intcode(&mut prg);
    let grid = grid_from_prg(&prg);

    sum_inter(&grid) as i64
}

enum Order {
    Forward(usize),
    R,
    L,
}

enum Orientation {
    Left,
    Top,
    Right,
    Bottom,
}

fn get_path(map: &Vec<Vec<usize>>) -> Vec<Order> {
    let mut p = vec![];
    let (h, w) = (map.len(), map[0].len());

    let (mut x, mut y) = (0, 0);
    let mut orientation = Orientation::Top;
    let orientations = ['>', '<', 'v', '^'].map(|c| c);

    for (i, j) in (0..h).cartesian_product(0..w) {
        if (orientations.contains(map[i][j])) {}
    }

    todo!();

    p
}

pub fn result_2(input: InputType) -> i64 {
    let init_instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let mut instrs = init_instrs.clone();
    assert!(instrs[&0usize] == 1);
    instrs.insert(0usize, 2);

    let mut init_prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: init_instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    run_intcode(&mut init_prg);
    let grid = grid_from_prg(&init_prg);

    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input[0].split(",").map(|x| x.parse().unwrap()).collect()
}

use std::collections::HashMap;

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

pub fn part1(s: String) -> Solution {
    let input = parse(s);

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
    run_intcode(&mut prg);

    Solution::from(
        prg.outputs
            .into_iter()
            .enumerate()
            .filter(|&(i, x)| i % 3 == 2 && x == 2)
            .count() as i64,
    )
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    instrs.insert(0, 2);

    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: Vec::from([]),
        outputs: Vec::new(),
    };

    run_intcode(&mut prg);

    let mut current_paddle_x = -1;
    let mut current_ball_x = -1;
    let mut current_score = -1;

    for k in 0..(prg.outputs.len() / 3) {
        let i = 3 * k;
        match prg.outputs[i + 2] {
            3 => {
                current_paddle_x = prg.outputs[i];
            }
            4 => {
                current_ball_x = prg.outputs[i];
            }
            _ => (),
        }
    }

    let mut remaining_blocks_count = prg
        .outputs
        .iter()
        .enumerate()
        .filter(|&(i, &x)| i % 3 == 2 && x == 2)
        .count();

    while remaining_blocks_count > 0 {
        // println!(
        //     "ball : {} {}\npaddle: {} {}\nremaining blocks: {}",
        //     current_ball_x,
        //     current_ball_y,
        //     current_paddle_x,
        //     current_paddle_y,
        //     remaining_blocks_count
        // );
        let inputs = Vec::from([if current_paddle_x < current_ball_x {
            1
        } else if current_paddle_x > current_ball_x {
            -1
        } else {
            0
        }]);

        prg.outputs = Vec::new();
        prg.inputs = inputs;

        run_intcode(&mut prg);

        for k in 0..(prg.outputs.len() / 3) {
            let i = 3 * k;
            if (prg.outputs[i], prg.outputs[i + 1]) == (-1, 0) {
                current_score = prg.outputs[i + 2];
                remaining_blocks_count -= 1;
                continue;
            }

            match prg.outputs[i + 2] {
                3 => {
                    current_paddle_x = prg.outputs[i];
                }
                4 => {
                    current_ball_x = prg.outputs[i];
                }
                _ => (),
            }
        }
    }

    Solution::from(current_score as i64)
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

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

fn is_light(instrs: HashMap<usize, i128>, x: i128, y: i128) -> bool {
    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: instrs.clone(),
        inputs: Vec::from([x, y]),
        outputs: vec![],
    };

    run_intcode(&mut prg);

    match prg.outputs.pop().unwrap() {
        0 => false,
        1 => true,
        _ => panic!(),
    }
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let mut count = 0;

    for j in 0..50 {
        for i in 0..50 {
            if is_light(instrs.clone(), i, j) {
                count += 1;
            }
        }
    }

    Solution::from(count)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();
    let mut y = 0;
    let mut x = 500;

    loop {
        // first did checking of diagonal,
        // bu then realised only the check of the four corners is required
        x += 1;

        while !is_light(instrs.clone(), x, y) {
            y += 1;
        }

        if is_light(instrs.clone(), x - 99, y)
            && is_light(instrs.clone(), x, y + 99)
            && is_light(instrs.clone(), x - 99, y + 99)
        {
            return Solution::from(((x - 99) + y * 10000) as i64);
        }
    }
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

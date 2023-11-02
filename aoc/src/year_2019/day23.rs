use std::collections::{HashMap, VecDeque};

use crate::Solution;

type InputType = Vec<i128>;

#[derive(Clone)]
struct Prg {
    i: i128,
    relative_base: i128,
    instrs: HashMap<usize, i128>,
    inputs: VecDeque<i128>,
    outputs: VecDeque<i128>,
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
                if let Some(x) = prg.inputs.pop_front() {
                    prg.set(args[0], x, modes[0]);
                } else {
                    return;
                }
                prg.i += 2;
            }
            4 => {
                // output
                prg.outputs.push_back(prg.eval_i(prg.i + 1, modes[0]));
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

    let prg_template = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: VecDeque::new(),
        outputs: VecDeque::new(),
    };

    let mut prgs = vec![prg_template; 50];
    for i in 0..50 {
        prgs[i].inputs.push_back(i as i128);
    }

    loop {
        for i in 0..50 {
            run_intcode(&mut prgs[i]);
        }

        let mut queues = vec![VecDeque::new(); 50];

        for i in 0..50 {
            while !prgs[i].outputs.is_empty() {
                let (address, x, y) = (
                    prgs[i].outputs.pop_front().unwrap(),
                    prgs[i].outputs.pop_front().unwrap(),
                    prgs[i].outputs.pop_front().unwrap(),
                );

                if address == 255 {
                    return Solution::from(y as i64);
                }

                queues[address as usize].push_back(x);
                queues[address as usize].push_back(y);
            }
        }

        for i in 0..50 {
            if queues[i].is_empty() {
                queues[i].push_back(-1);
            }

            prgs[i].inputs = queues[i].clone();
        }
    }
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let prg_template = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: VecDeque::new(),
        outputs: VecDeque::new(),
    };

    let mut current_nat_packet = None;

    let mut prgs = vec![prg_template; 50];
    for i in 0..50 {
        prgs[i].inputs.push_back(i as i128);
    }

    let mut idle_count = 0;
    let mut already_sent = false;

    loop {
        for i in 0..50 {
            run_intcode(&mut prgs[i]);
        }

        let mut queues = vec![VecDeque::new(); 51];
        let mut network_is_idle = true;

        for i in 0..50 {
            while !prgs[i].outputs.is_empty() {
                network_is_idle = false;
                let (address, x, y) = (
                    prgs[i].outputs.pop_front().unwrap(),
                    prgs[i].outputs.pop_front().unwrap(),
                    prgs[i].outputs.pop_front().unwrap(),
                );

                if address == 255 {
                    already_sent = false;

                    current_nat_packet = Some((x, y));

                    continue;
                }

                queues[address as usize].push_back(x);
                queues[address as usize].push_back(y);
            }
        }

        if network_is_idle {
            idle_count += 1;
        }
        if idle_count > 2 {
            if let Some((x, y)) = current_nat_packet {
                if already_sent {
                    return Solution::from(y as i64);
                }
                queues[0].push_back(x);
                queues[0].push_back(y);
            } else {
                panic!();
            }
            idle_count = 0;
            already_sent = true;
        }

        for i in 0..50 {
            if queues[i].is_empty() {
                queues[i].push_back(-1);
            }

            prgs[i].inputs = queues[i].clone();
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

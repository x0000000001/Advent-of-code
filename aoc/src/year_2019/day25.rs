use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

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

fn str_to_ascii(s: &str) -> VecDeque<i128> {
    s.chars().map(|c| c as u8 as i128).collect()
}

fn ascii_to_str(l: &VecDeque<i128>) -> String {
    l.iter()
        .map(|&c| if c < 256 { c as u8 as char } else { ' ' })
        .collect()
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
        inputs: VecDeque::new(),
        outputs: VecDeque::new(),
    };

    let names = [
        "food ration",
        "mouse",
        "mug",
        "ornament",
        "mutex",
        "candy cane",
        "coin",
        "semiconductor",
    ];

    let init_commands = [
        "north",
        "take mug",
        "north",
        "take food ration",
        "south",
        "east",
        "north",
        "east",
        "take semiconductor",
        "west",
        "south",
        "west",
        "south",
        "east",
        "take ornament",
        "north",
        "take coin",
        "east",
        "take mutex",
        "west",
        "south",
        "east",
        "take candy cane",
        "west",
        "west",
        "south",
        "east",
        "take mouse",
        "south",
    ];

    prg.inputs.append(&mut str_to_ascii(
        &init_commands
            .into_iter()
            .fold(String::new(), |acc, s| format!("{acc}{s}\n")),
    ));
    run_intcode(&mut prg);

    // TODO fix this day : it is stuck in an infinite loop
    loop {
        // println!("{}", ascii_to_str(&prg.outputs));
        prg.outputs.clear();
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).unwrap();

        if line == "bruteforce\n" {
            // println!("bruteforcing !");
            for mut i in 0..255 {
                let mut s: String = names
                    .iter()
                    .map(|name| format!("drop {}\n", name))
                    .collect();

                for j in 0..8 {
                    if i % 2 == 1 {
                        s = format!("{s}take {}\n", names[j])
                    }

                    i /= 2;
                }

                s = format!("{s}west\n");
                prg.inputs.append(&mut str_to_ascii(&s));
                run_intcode(&mut prg);

                let answer = ascii_to_str(&prg.outputs);
                prg.outputs.clear();
                // if answer.contains("password") {
                return Solution::from(answer);
                // return 0;
                // }
            }
        } else {
            prg.inputs.append(&mut str_to_ascii(&line));
        }

        run_intcode(&mut prg);
    }
}

pub fn part2(_: String) -> Solution {
    Solution::Day25Part2
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

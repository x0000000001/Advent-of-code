use std::fs;

pub type InputType = Vec<i64>;

struct Prg {
    i: i64,
    relative_base: i64,
    instrs: Vec<i64>,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

#[derive(Clone, Copy)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Prg {
    fn eval_i(&self, i: i64, m: Mode) -> i64 {
        match m {
            Mode::Immediate => i,
            Mode::Position => self.instrs[i as usize],
            Mode::Relative => self.instrs[(i + self.relative_base) as usize],
        }
    }
}

fn run_intcode(prg: &mut Prg) {
    while prg.i >= 0 && prg.i < prg.instrs.len() as i64 {
        let instr = prg.instrs[prg.i as usize];
        let code = instr % 100;
        let modes_str = (code - (code % 100)).to_string();
        let mut modes: Vec<Mode> = vec![Mode::Immediate; 3];
        let args = [
            prg.instrs[(prg.i + 1) as usize],
            prg.instrs[(prg.i + 2) as usize],
            prg.instrs[(prg.i + 3) as usize],
        ];

        for (i, c) in modes_str.chars().enumerate() {
            modes[i] = match c {
                '0' => Mode::Immediate,
                '1' => Mode::Position,
                '2' => Mode::Relative,
                _ => panic!(),
            }
        }

        match code {
            99 => return,
            1 => {
                prg.instrs[args[2] as usize] =
                    prg.eval_i(prg.i + 1, modes[0]) + prg.eval_i(prg.i + 2, modes[1]);
                prg.i += 4;
            }
            2 => {
                prg.instrs[args[2] as usize] =
                    prg.eval_i(prg.i + 1, modes[0]) * prg.eval_i(prg.i + 2, modes[1]);
                prg.i += 4;
            }
            3 => {
                if let Some(x) = prg.inputs.pop() {
                    prg.instrs[args[0] as usize] = x;
                } else {
                    return;
                }
                prg.i += 2;
            }
            4 => {
                prg.outputs.push(prg.eval_i(prg.i + 1, modes[0]));
                prg.i += 2;
            }
            5 => {
                if prg.eval_i(prg.i + 1, modes[0]) != 0 {
                    prg.i += prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            6 => {
                if prg.eval_i(prg.i + 1, modes[0]) == 0 {
                    prg.i += prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            7 => {
                if prg.eval_i(prg.i + 1, modes[0]) < prg.eval_i(prg.i + 2, modes[1]) {
                    prg.instrs[args[2] as usize] = 1;
                } else {
                    prg.instrs[args[2] as usize] = 0;
                }
                prg.i += 4;
            }
            8 => {
                if prg.eval_i(prg.i + 1, modes[0]) == prg.eval_i(prg.i + 2, modes[1]) {
                    prg.instrs[args[2] as usize] = 1;
                } else {
                    prg.instrs[args[2] as usize] = 0;
                }
                prg.i += 4;
            }
            9 => {
                prg.relative_base += prg.eval_i(prg.i + 1, modes[0]);
                prg.i += 2;
            }
            _ => panic!(),
        }
    }
}

pub fn result_1(instrs: InputType) -> i64 {
    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    run_intcode(&mut prg);
    println!("{:?}", prg.outputs);

    0
}

pub fn result_2(input: InputType) -> i64 {
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

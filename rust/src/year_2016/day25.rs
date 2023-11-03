use crate::Solution;

// cpy : 0
// inc : 1
// dec : 2
// jnz : 3
// out : 4
// a,b,c,d : 0,1,2,3
type InputType = Vec<(u64, InstrVal, InstrVal)>;
const CHECKS: u64 = 1000;

#[derive(Clone, Copy)]
pub enum InstrVal {
    Val(i64),
    Index(usize),
    None,
}
// returns whether input returns 0,1,0,1...
fn run_instr(reg: &mut [i64], instructions: &InputType) -> bool {
    let mut index = 0;
    let mut must_be_0 = true;
    let mut check_count = 0;

    'instr_loop: while index < instructions.len() {
        let (instr, x, y) = instructions[index];

        match instr {
            0 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => panic!("Can't copy wrong value."),
                };

                match y {
                    InstrVal::Index(i) => reg[i] = val,
                    _ => panic!("Wrong register to copy in."),
                }
            }

            1 => match x {
                InstrVal::Index(i) => reg[i] += 1,
                _ => panic!("Wrong register to increment."),
            },

            2 => match x {
                InstrVal::Index(i) => reg[i] -= 1,
                _ => panic!("Wrong register to increment."),
            },

            3 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => panic!("Can't copy wrong value."),
                };

                if val == 0 {
                    index += 1;
                    continue 'instr_loop;
                }

                match y {
                    InstrVal::Val(i) => {
                        index = (index as i64 + i) as usize;
                        continue 'instr_loop;
                    }
                    _ => panic!("Wrong register to increment."),
                }
            }

            4 => {
                let val = match x {
                    InstrVal::Val(i) => i,
                    InstrVal::Index(i) => reg[i],
                    _ => panic!("Can't print None"),
                };
                if (must_be_0 && val == 0) || (!must_be_0 && val == 1) {
                    must_be_0 = !must_be_0;
                    check_count += 1;
                    if check_count > CHECKS {
                        return true;
                    }
                } else {
                    return false;
                }
            }

            _ => panic!("Unknown instruction : {}", instr),
        }

        index += 1;
    }

    false
}

pub fn part1(s: String) -> Solution {
    let instructions = parse(s);

    let mut i = 0;

    loop {
        let mut reg = [i, 1, 2, 3];
        if run_instr(&mut reg, &instructions) {
            return Solution::from(i as i64);
        }
        i += 1;
    }
}

pub fn part2(_: String) -> Solution {
    Solution::Day25Part2
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

        let get_instr_val = |inp: &str| -> InstrVal {
            if let Ok(val) = inp.parse() {
                InstrVal::Val(val)
            } else {
                InstrVal::Index(match inp {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    _ => panic!("Wrong index."),
                })
            }
        };

        let x = get_instr_val(w[1]);

        let mut y: InstrVal = InstrVal::None;

        let instr = match w[0] {
            "cpy" => 0,
            "inc" => 1,
            "dec" => 2,
            "jnz" => 3,
            "out" => 4,
            _ => panic!("Wrong instruction."),
        };

        if [0, 3].contains(&instr) {
            y = get_instr_val(w[2]);
        }

        res.push((instr, x, y));
    }

    res
}

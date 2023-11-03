use crate::Solution;

type InputType = Vec<(i64, i64, i64)>;

pub fn part1(s: String) -> Solution {
    let instructions = parse(s);

    let mut i = 0;
    let mut var = [0, 0];

    while i < instructions.len() {
        let (instr, arg0, arg1) = instructions[i];
        match instr {
            0 => var[arg0 as usize] /= 2,
            1 => var[arg0 as usize] *= 3,
            2 => var[arg0 as usize] += 1,
            3 => {
                i = (i as i64 + arg0) as usize;
                continue;
            }
            4 => {
                if var[arg0 as usize] % 2 == 0 {
                    i += arg1 as usize;
                    continue;
                }
            }
            5 => {
                if var[arg0 as usize] == 1 {
                    i += arg1 as usize;
                    continue;
                }
            }
            _ => (),
        }

        i += 1;
    }

    Solution::from(var[1])
}

pub fn part2(s: String) -> Solution {
    let instructions = parse(s);

    let mut i = 0;
    let mut var = [1, 0];

    while i < instructions.len() {
        let (instr, arg0, arg1) = instructions[i];
        match instr {
            0 => var[arg0 as usize] /= 2,
            1 => var[arg0 as usize] *= 3,
            2 => var[arg0 as usize] += 1,
            3 => {
                i = (i as i64 + arg0) as usize;
                continue;
            }
            4 => {
                if var[arg0 as usize] % 2 == 0 {
                    i += arg1 as usize;
                    continue;
                }
            }
            5 => {
                if var[arg0 as usize] == 1 {
                    i += arg1 as usize;
                    continue;
                }
            }
            _ => (),
        }

        i += 1;
    }

    Solution::from(var[1])
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        let instr = match w[0] {
            "hlf" => 0,
            "tpl" => 1,
            "inc" => 2,
            "jmp" => 3,
            "jie" => 4,
            "jio" => 5,
            _ => -1,
        };

        let arg0 = if instr == 3 {
            w[1].parse().unwrap()
        } else {
            match w[1] {
                "a," => 0,
                "b," => 1,
                "a" => 0,
                "b" => 1,
                _ => -1,
            }
        };

        let arg1 = if instr == 4 || instr == 5 {
            w[2].parse().unwrap()
        } else {
            -1
        };

        res.push((instr, arg0, arg1));
    }

    res
}

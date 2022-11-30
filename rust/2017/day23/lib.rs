use std::fs;

#[derive(Clone, Copy)]
pub enum Var {
    Num(i64),
    Index(usize),
}

impl Var {
    fn eval(&self, regs: &Vec<i64>) -> i64 {
        match self {
            Var::Num(x) => *x,
            Var::Index(i) => regs[*i],
        }
    }

    fn from(s: &str) -> Var {
        if let Ok(x) = s.parse::<i64>() {
            Var::Num(x)
        } else {
            Var::Index(match s.chars().next().unwrap() {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => panic!(),
            })
        }
    }

    fn get_as_id(&self) -> usize {
        match self {
            Var::Num(_) => panic!(),
            Var::Index(x) => *x as usize,
        }
    }

    fn get_val(&self) -> i64 {
        match self {
            Var::Num(x) => *x,
            Var::Index(_) => panic!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instr {
    Set(Var, Var),
    Sub(Var, Var),
    Mul(Var, Var),
    Jnz(Var, Var),
}

pub type InputType = Vec<Instr>;

pub fn result_1(prg: InputType) -> i64 {
    let mut count = 0;
    let mut i: i64 = 0;
    let mut regs = vec![0; 8];

    while i >= 0 && i < prg.len() as i64 {
        match prg[i as usize] {
            Instr::Set(x, y) => {
                regs[x.get_as_id()] = y.eval(&regs);
                i += 1;
            }
            Instr::Sub(x, y) => {
                regs[x.get_as_id()] -= y.eval(&regs);
                i += 1;
            }
            Instr::Mul(x, y) => {
                count += 1;
                regs[x.get_as_id()] *= y.eval(&regs);
                i += 1;
            }
            Instr::Jnz(x, y) => {
                if x.eval(&regs) != 0 {
                    i += y.eval(&regs);
                } else {
                    i += 1;
                }
            }
        }
    }

    count
}

// for testing purposes
#[allow(dead_code)]
fn run_prog(prg: InputType) {
    let mut i: i64 = 0;
    let mut regs = vec![0; 8];
    regs[0] = 1;

    while i >= 0 && i < prg.len() as i64 {
        if i == 9 {
            print!("")
        }

        if i == 10 {
            print!("")
        }

        if i == 11 {
            print!("")
        }

        match prg[i as usize] {
            Instr::Set(x, y) => {
                regs[x.get_as_id()] = y.eval(&regs);
                i += 1;
            }
            Instr::Sub(x, y) => {
                regs[x.get_as_id()] -= y.eval(&regs);
                i += 1;
            }
            Instr::Mul(x, y) => {
                regs[x.get_as_id()] *= y.eval(&regs);
                i += 1;
            }
            Instr::Jnz(x, y) => {
                if x.eval(&regs) != 0 {
                    i += y.eval(&regs);
                } else {
                    i += 1;
                }
            }
        }
    }
}

impl Instr {
    fn get_arg(&self, n: usize) -> i64 {
        match (self, n) {
            (Instr::Set(x, _), 0) => x.get_val(),
            (Instr::Set(_, y), 1) => y.get_val(),
            (Instr::Sub(x, _), 0) => x.get_val(),
            (Instr::Sub(_, y), 1) => y.get_val(),
            (Instr::Mul(x, _), 0) => x.get_val(),
            (Instr::Mul(_, y), 1) => y.get_val(),
            (Instr::Jnz(x, _), 0) => x.get_val(),
            (Instr::Jnz(_, y), 1) => y.get_val(),
            _ => panic!(),
        }
    }
}

pub fn result_2(prg: InputType) -> i64 {
    // run_prog(prg.clone());

    let param0 = prg[0].get_arg(1);
    let param1 = prg[4].get_arg(1);
    let param2 = prg[5].get_arg(1);
    let param3 = prg[7].get_arg(1);

    let mut b = (param0 * param1) - param2;
    let c = b - param3;

    let mut h = 0;

    while b != (c + 17) {
        let mut f = false;
        for d in 2..(b + 1) {
            if b % d == 0 && b / d >= 2 && b / d <= b {
                f = true;
                break;
            }
        }
        if f {
            h += 1;
        }

        b += 17;
    }

    h
}

// 904 too low
fn line_to_instr(l: &str) -> Instr {
    let words = l.split_whitespace().collect::<Vec<&str>>();
    let arg0 = Var::from(words[1]);
    let arg1 = Var::from(words[2]);

    match words[0] {
        "set" => Instr::Set(arg0, arg1),
        "sub" => Instr::Sub(arg0, arg1),
        "mul" => Instr::Mul(arg0, arg1),
        "jnz" => Instr::Jnz(arg0, arg1),
        _ => panic!(),
    }
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input.into_iter().map(|x| line_to_instr(&x)).collect()
}
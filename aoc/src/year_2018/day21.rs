use std::collections::HashSet;

use crate::Solution;

type InputType = Machine;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    Addr(usize, usize, usize),
    Addi(usize, usize, usize),
    Mulr(usize, usize, usize),
    Muli(usize, usize, usize),
    Banr(usize, usize, usize),
    Bani(usize, usize, usize),
    Borr(usize, usize, usize),
    Bori(usize, usize, usize),
    Setr(usize, usize, usize),
    Seti(usize, usize, usize),
    Gtir(usize, usize, usize),
    Gtri(usize, usize, usize),
    Gtrr(usize, usize, usize),
    Eqir(usize, usize, usize),
    Eqri(usize, usize, usize),
    Eqrr(usize, usize, usize),
}

/////////
// INSTRS
/////////

fn addr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] + regs[y];
}

fn addi(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] + y;
}

fn mulr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] * regs[y];
}

fn muli(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] * y;
}

fn banr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] & regs[y];
}

fn bani(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] & y;
}

fn borr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] | regs[y];
}

fn bori(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x] | y;
}

fn setr(x: usize, _: usize, z: usize, regs: &mut Regs) {
    regs[z] = regs[x];
}

fn seti(x: usize, _: usize, z: usize, regs: &mut Regs) {
    regs[z] = x;
}

fn gtir(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if x > regs[y] { 1 } else { 0 };
}

fn gtri(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if regs[x] > y { 1 } else { 0 };
}

fn gtrr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if regs[x] > regs[y] { 1 } else { 0 };
}

fn eqir(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if x == regs[y] { 1 } else { 0 };
}

fn eqri(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if regs[x] == y { 1 } else { 0 };
}

fn eqrr(x: usize, y: usize, z: usize, regs: &mut Regs) {
    regs[z] = if regs[x] == regs[y] { 1 } else { 0 };
}

#[allow(dead_code)]
fn ip(x: usize, state: &mut Machine) {
    state.instr_pointer_index = x;
}

/////////
// \INSTRS
/////////

type Regs = Vec<usize>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Machine {
    regs: Regs,
    instr_pointer_index: usize,
    instrs: Vec<Instr>,
    instructions_count: usize,
}

#[allow(dead_code)]
impl Machine {
    fn run(&mut self) -> bool {
        let instr_pointer = self.regs[self.instr_pointer_index];

        match self.instrs[instr_pointer] {
            Instr::Addr(x, y, z) => addr(x, y, z, &mut self.regs),
            Instr::Addi(x, y, z) => addi(x, y, z, &mut self.regs),
            Instr::Mulr(x, y, z) => mulr(x, y, z, &mut self.regs),
            Instr::Muli(x, y, z) => muli(x, y, z, &mut self.regs),
            Instr::Banr(x, y, z) => banr(x, y, z, &mut self.regs),
            Instr::Bani(x, y, z) => bani(x, y, z, &mut self.regs),
            Instr::Borr(x, y, z) => borr(x, y, z, &mut self.regs),
            Instr::Bori(x, y, z) => bori(x, y, z, &mut self.regs),
            Instr::Setr(x, y, z) => setr(x, y, z, &mut self.regs),
            Instr::Seti(x, y, z) => seti(x, y, z, &mut self.regs),
            Instr::Gtir(x, y, z) => gtir(x, y, z, &mut self.regs),
            Instr::Gtri(x, y, z) => gtri(x, y, z, &mut self.regs),
            Instr::Gtrr(x, y, z) => gtrr(x, y, z, &mut self.regs),
            Instr::Eqir(x, y, z) => eqir(x, y, z, &mut self.regs),
            Instr::Eqri(x, y, z) => eqri(x, y, z, &mut self.regs),
            Instr::Eqrr(x, y, z) => eqrr(x, y, z, &mut self.regs),
        }

        self.instructions_count += 1;

        let instr_pointer = self.regs[self.instr_pointer_index];

        if instr_pointer >= self.instrs.len() - 1 {
            return false;
        } else {
            self.regs[self.instr_pointer_index] += 1;
            true
        }
    }

    fn new() -> Machine {
        Machine {
            regs: vec![0; 6],
            instr_pointer_index: 0,
            instrs: vec![],
            instructions_count: 0,
        }
    }
}

/// Desassembly of the input.
#[allow(dead_code)]
fn fun(x0: usize) {
    let mut x1;
    let mut x2;
    let mut x4 = 0;

    loop {
        x2 = x4 | 0b10000000000000000;
        x4 = 0b10111011110000001011101;

        loop {
            x4 += x2 & 0b11111111;
            x4 &= 0b111111111111111111111111;
            x4 *= 0b10000000101101011;
            x4 &= 0b111111111111111111111111;

            if x2 < 256 {
                break;
            }

            x1 = 0;
            loop {
                if 256 * (x1 + 1) > x2 {
                    break;
                }

                x1 += 1;
            }

            x2 = x1;
        }

        if x4 == x0 {
            break;
        }
    }
}

pub fn part1(_s: String) -> Solution {
    // let input = parse(s);

    let mut x1;
    let mut x2;
    let mut x4;

    x2 = 65536;
    x4 = 6152285;

    loop {
        x4 += x2 & 255;
        x4 &= 16777215;
        x4 *= 65899;
        x4 &= 16777215;

        if x2 < 256 {
            break;
        }

        x1 = (x2 / 256) - 1;
        loop {
            if 256 * (x1 + 1) > x2 {
                break;
            }

            x1 += 1;
        }

        x2 = x1;
    }

    Solution::from(x4)
}

pub fn part2(_s: String) -> Solution {
    // let input = parse(s);

    let mut x1;
    let mut x2;
    let mut x4 = 0;

    let mut seen = HashSet::new();
    let mut previous;

    loop {
        previous = x4;

        x2 = x4 | 0b10000000000000000;
        x4 = 0b10111011110000001011101;

        loop {
            x4 += x2 & 0b11111111;
            x4 &= 0b111111111111111111111111;
            x4 *= 0b10000000101101011;
            x4 &= 0b111111111111111111111111;

            if x2 < 256 {
                break;
            }

            x1 = 0;
            loop {
                if 256 * (x1 + 1) > x2 {
                    break;
                }

                x1 += 1;
            }

            x2 = x1;
        }

        if seen.contains(&x4) {
            return Solution::from(previous);
        } else {
            seen.insert(x4);
        }
    }
}

#[allow(dead_code)]
fn parse(s: String) -> InputType {
    let instr_from_string = |l: &str| -> Instr {
        let words = l.split_whitespace().collect::<Vec<&str>>();
        let args: Vec<usize> = words[1..].iter().map(|w| w.parse().unwrap()).collect();

        match words[0] {
            "addr" => Instr::Addr(args[0], args[1], args[2]),
            "addi" => Instr::Addi(args[0], args[1], args[2]),
            "mulr" => Instr::Mulr(args[0], args[1], args[2]),
            "muli" => Instr::Muli(args[0], args[1], args[2]),
            "banr" => Instr::Banr(args[0], args[1], args[2]),
            "bani" => Instr::Bani(args[0], args[1], args[2]),
            "borr" => Instr::Borr(args[0], args[1], args[2]),
            "bori" => Instr::Bori(args[0], args[1], args[2]),
            "setr" => Instr::Setr(args[0], args[1], args[2]),
            "seti" => Instr::Seti(args[0], args[1], args[2]),
            "gtir" => Instr::Gtir(args[0], args[1], args[2]),
            "gtri" => Instr::Gtri(args[0], args[1], args[2]),
            "gtrr" => Instr::Gtrr(args[0], args[1], args[2]),
            "eqir" => Instr::Eqir(args[0], args[1], args[2]),
            "eqri" => Instr::Eqri(args[0], args[1], args[2]),
            "eqrr" => Instr::Eqrr(args[0], args[1], args[2]),
            _ => panic!(),
        }
    };

    let mut input = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty());

    let ip = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let instrs = input.map(|l| instr_from_string(&l)).collect();

    let mut m = Machine::new();
    m.instrs = instrs;
    m.instr_pointer_index = ip;

    m
}

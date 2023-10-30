use std::fs;

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

type Regs = Vec<usize>;

pub struct Machine {
    regs: Regs,
    instr_pointer_index: usize,
    instrs: Vec<Instr>,
}

pub type InputType = Machine;

impl Machine {
    fn run(&mut self) -> bool {
        let instr_pointer = self.regs[self.instr_pointer_index];

        println!("{}", instr_pointer);

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
        }
    }
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

fn ip(x: usize, state: &mut Machine) {
    state.instr_pointer_index = x;
}

/////////
// \INSTRS
/////////

fn get_n(instrs: &Vec<Instr>, x0_1: bool) -> usize {
    let h0 = if let Instr::Addi(_, y, _) = instrs[17] {
        y
    } else {
        panic!()
    };

    let h1 = if let Instr::Muli(_, y, _) = instrs[20] {
        y
    } else {
        panic!()
    };

    let h2 = if let Instr::Addi(_, y, _) = instrs[21] {
        y
    } else {
        panic!()
    };

    let h3 = if let Instr::Addi(_, y, _) = instrs[23] {
        y
    } else {
        panic!()
    };

    let h4 = if let Instr::Muli(_, y, _) = instrs[31] {
        y
    } else {
        panic!()
    };

    if !x0_1 {
        h0 * h0 * 19 * h1 + h2 * 22 + h3
    } else {
        h0 * h0 * 19 * h1 + h2 * 22 + h3 + (27 * 28 + 29) * 30 * h4 * 32
    }
}

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    let limit = (f64::sqrt(n as f64) as usize) + 2;

    for i in 1..(limit + 1) {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }

    sum
}

/// It's a sum of divisors in O(n^2). We compute n and do it faster.
pub fn result_1(machine: InputType) -> i64 {
    sum_of_divisors(get_n(&machine.instrs, false)) as i64
}

// letting it here for the record
// not correct though, some indices condition must be wrong
// anyway, we guessed what it's willing to do :)
// fn traduction(x0_1: bool) -> usize {
//     let mut x4;
//     x4 = 4 * 19 * 11 + 4 * 22 + 2;

//     if x0_1 {
//         x4 += (27 * 28 + 29) * 30 * 14 * 32;
//     }

//     let mut res = 0;

//     for x2 in 1..(x4 + 1) {
//         for x5 in 1..(x2 + 1) {
//             if x2 * x5 == x4 {
//                 res += x2;
//             }
//         }
//     }

//     res
// }

pub fn result_2(mut machine: InputType) -> i64 {
    sum_of_divisors(get_n(&machine.instrs, true)) as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

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

    let mut input = contents
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

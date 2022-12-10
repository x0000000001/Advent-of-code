use std::{collections::HashSet, fs};

type Instr = Vec<usize>;
type State = Vec<usize>;
pub struct Test {
    before: State,
    instr: Instr,
    after: State,
}

pub type InputType = (Vec<Test>, Vec<Instr>);
type InstrFunc = fn(&Instr, &mut State);

/////////
// INSTRS
/////////

fn addr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] + regs[instr[2]];
}

fn addi(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] + instr[2];
}

fn mulr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] * regs[instr[2]];
}

fn muli(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] * instr[2];
}

fn banr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] & regs[instr[2]];
}

fn bani(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] & instr[2];
}

fn borr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] | regs[instr[2]];
}

fn bori(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]] | instr[2];
}

fn setr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = regs[instr[1]];
}

fn seti(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = instr[1];
}

fn gtir(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if instr[1] > regs[instr[2]] { 1 } else { 0 };
}

fn gtri(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if regs[instr[1]] > instr[2] { 1 } else { 0 };
}

fn gtrr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if regs[instr[1]] > regs[instr[2]] {
        1
    } else {
        0
    };
}

fn eqir(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if instr[1] == regs[instr[2]] { 1 } else { 0 };
}

fn eqri(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if regs[instr[1]] == instr[2] { 1 } else { 0 };
}

fn eqrr(instr: &Instr, regs: &mut State) {
    regs[instr[3]] = if regs[instr[1]] == regs[instr[2]] {
        1
    } else {
        0
    };
}

/////////
// \INSTRS
/////////

const FUNCS: [InstrFunc; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

pub fn result_1((tests, _): InputType) -> i64 {
    tests
        .into_iter()
        .map(|t| {
            FUNCS
                .iter()
                .map(|f| {
                    let mut result = t.before.clone();
                    f(&t.instr, &mut result);
                    result
                })
                .filter(|r| r.eq(&t.after))
                .count()
        })
        .filter(|count| *count >= 3)
        .count() as i64
}

pub fn result_2((tests, instrs): InputType) -> i64 {
    let mut possible_corresponding = (0..FUNCS.len())
        .into_iter()
        .map(|i| {
            let mut possible_codes = (0..16).collect::<HashSet<usize>>();
            tests.iter().for_each(|t| {
                let mut result = t.before.clone();
                FUNCS[i](&t.instr, &mut result);
                if !t.after.eq(&result) {
                    possible_codes.remove(&t.instr[0]);
                }
            });

            possible_codes
        })
        .collect::<Vec<HashSet<usize>>>();

    let mut found = vec![false; FUNCS.len()];

    while found.clone().into_iter().reduce(|a, b| a && b).unwrap() != true {
        for i in 0..FUNCS.len() {
            if !found[i] && possible_corresponding[i].len() == 1 {
                let new_val = *possible_corresponding[i].iter().next().unwrap();
                found[i] = true;
                for j in 0..FUNCS.len() {
                    if i == j {
                        continue;
                    }
                    possible_corresponding[j].remove(&new_val);
                }
                break;
            }
        }
    }

    let mut bindings = [0; FUNCS.len()];

    possible_corresponding
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| bindings[s.into_iter().next().unwrap()] = i);

    let mut state = vec![0; 4];

    for instr in instrs {
        FUNCS[bindings[instr[0]]](&instr, &mut state);
        println!("{:?} instr {:?}", state, instr);
    }

    state[0] as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let mut tests: Vec<Test> = vec![];
    let mut i = 0;

    let state_from_string = |l: &str| -> Instr {
        let lp = l.replace(", ", ",");
        let words1 = lp.split_whitespace().collect::<Vec<&str>>();

        words1[1][1..words1[1].len() - 1]
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect()
    };

    let instr_from_string = |l: &str| -> Instr {
        l[0..l.len()]
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect()
    };

    loop {
        let mut words = input[i].split_whitespace();
        if words.next().unwrap() != "Before:" {
            break;
        }

        tests.push({
            Test {
                before: state_from_string(&input[i]),
                instr: instr_from_string(&input[i + 1]),
                after: state_from_string(&input[i + 2]),
            }
        });

        i += 3;
    }

    let mut instrs = vec![];

    while i < input.len() {
        instrs.push(instr_from_string(&input[i]));

        i += 1;
    }

    (tests, instrs)
}

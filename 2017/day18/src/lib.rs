use core::panic;
use std::{fs, collections::{HashMap, VecDeque}};

pub type InputType = Vec<Instr>;

#[derive(Debug)]
pub enum Val {
    Var(char),
    Num(i64)
}

impl Val {
    pub fn from_str(s: &str) -> Val {
        if let Ok(x) = s.parse::<i64>() {
            Val::Num(x)
        } else {
            Val::Var(s.chars().next().unwrap())
        }
    }

    pub fn evaluate(&self, vars: &mut HashMap<char,i64>) -> i64 {
        match self {
            Val::Num(x) => *x,
            Val::Var(c) => {
                let entry = vars.entry(*c).or_insert(0);
                *entry
            }
        }
    }

    pub fn set(&self, val: &Val, vars: &mut HashMap<char,i64>) {
        match self {
            Val::Num(_) => panic!(),
            Val::Var(c) => {
                let y = val.evaluate(vars);
                let entry = vars.entry(*c).or_insert(0);
                *entry = y;
            }
        }
    }

    pub fn add(&self, val: &Val, vars: &mut HashMap<char,i64>) {
        match self {
            Val::Num(_) => panic!(),
            Val::Var(c) => {
                let y = val.evaluate(vars);
                let entry = vars.entry(*c).or_insert(0);
                *entry += y;
            }
        }
    }

    pub fn mul(&self, val: &Val, vars: &mut HashMap<char,i64>) {
        match self {
            Val::Num(_) => panic!(),
            Val::Var(c) => {
                let y = val.evaluate(vars);
                let entry = vars.entry(*c).or_insert(0);
                *entry *= y;
            }
        }
    }

    pub fn modulo(&self, val: &Val, vars: &mut HashMap<char,i64>) {
        match self {
            Val::Num(_) => panic!(),
            Val::Var(c) => {
                let y = val.evaluate(vars);
                let entry = vars.entry(*c).or_insert(0);
                *entry = *entry - y * (*entry/y);
            }
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Snd(Val),
    Set(Val,Val),
    Add(Val,Val),
    Mul(Val,Val),
    Mod(Val,Val),
    Rcv(Val),
    Jgz(Val,Val)
}

pub struct Machine {
    index: i64,
    is_waiting: bool,
    has_stopped: bool,
    queue: VecDeque<i64>,
    regs: HashMap<char,i64>,
    snd_count: u64
}

pub fn result_1(instrs: InputType) -> i64
{
    let mut vars: HashMap<char,i64> = HashMap::new();
    let mut index: i64 = 0;
    let mut last_sound_rcv = 0;

    while index >= 0 && index < instrs.len() as i64{

        let ins = &instrs[index as usize];

        match ins {
            Instr::Snd(x) => last_sound_rcv = x.evaluate(&mut vars),
            Instr::Set(x, y) => x.set(y, &mut vars),
            Instr::Add(x, y) => x.add(y, &mut vars),
            Instr::Mul(x, y) => x.mul(y, &mut vars),
            Instr::Mod(x, y) => x.modulo(y, &mut vars),
            Instr::Rcv(x) => if x.evaluate(&mut vars) != 0 {
                return last_sound_rcv;
            },
            Instr::Jgz(x, y) => if x.evaluate(&mut vars) != 0 {
                index += y.evaluate(&mut vars);
                continue;
            }
        }

        index += 1;
    }

    0
}

fn run_machine(machine: &mut Machine, instrs: &Vec<Instr>, send_queue: &mut VecDeque<i64>) {

    if machine.has_stopped {
        return;
    }

    if machine.is_waiting && machine.queue.is_empty() {
        return;
    }

    let ins = &instrs[machine.index as usize];

    if machine.is_waiting {
        if let Instr::Rcv(Val::Var(c)) = ins {
            let val = machine.queue.pop_front().unwrap();
            let entry = machine.regs.entry(*c).or_insert(0);
            *entry = val;

            machine.is_waiting = false;
            machine.index += 1;

            if machine.index < 0 || machine.index >= instrs.len() as i64 {
                machine.has_stopped = true;
            }

        } else {
            panic!();
        }
    }

    while !machine.is_waiting && !machine.has_stopped {
        let ins = &instrs[machine.index as usize];

        match ins {
            Instr::Snd(x) =>  {
                send_queue.push_back(x.evaluate(&mut machine.regs));
                machine.snd_count += 1;
            },
            Instr::Set(x, y) => x.set(&y, &mut machine.regs),
            Instr::Add(x, y) => x.add(&y, &mut machine.regs),
            Instr::Mul(x, y) => x.mul(&y, &mut machine.regs),
            Instr::Mod(x, y) => x.modulo(&y, &mut machine.regs),
            Instr::Rcv(x) => if let Some(val) = machine.queue.pop_front() {
                if let Val::Var(c) = x {
                    let entry = machine.regs.entry(*c).or_insert(0);
                    *entry = val;
                } else {
                    panic!();
                }
            } else {
                machine.is_waiting =  true;
                break;
            },
            Instr::Jgz(x, y) => if x.evaluate(&mut machine.regs) > 0 {
                machine.index += y.evaluate(&mut machine.regs) - 1; // -1 to compensate +1 after match
            }
        }

        machine.index += 1;

        if machine.index < 0 || machine.index >= instrs.len() as i64 {
            machine.has_stopped = true;
        }
    }
}

// 127 too low
pub fn result_2(instrs: InputType) -> i64
{   
    let mut machine0: Machine = Machine { 
        index: 0, 
        is_waiting: false, 
        has_stopped: false, 
        queue: VecDeque::new(), 
        regs: HashMap::new(), 
        snd_count: 0
    };

    let mut machine1: Machine = Machine { 
        index: 0, 
        is_waiting: false, 
        has_stopped: false, 
        queue: VecDeque::new(), 
        regs: HashMap::new(), 
        snd_count: 0
    };

    machine1.regs.insert('p', 1);

    while !((machine0.has_stopped || (machine0.is_waiting && machine0.queue.is_empty())) 
        && (machine1.has_stopped || (machine1.is_waiting && machine1.queue.is_empty()))) {
        run_machine(&mut machine0, &instrs, &mut machine1.queue);
        run_machine(&mut machine1, &instrs, &mut machine0.queue);
    }


    machine1.snd_count as i64
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut res: InputType = vec![];

    for line in input {
        let w = line.split_whitespace().collect::<Vec<&str>>();

        res.push(match w[0] {
            "snd" => Instr::Snd(Val::from_str(w[1])),
            "set" => Instr::Set(Val::from_str(w[1]), Val::from_str(w[2])),
            "add" => Instr::Add(Val::from_str(w[1]), Val::from_str(w[2])),
            "mul" => Instr::Mul(Val::from_str(w[1]), Val::from_str(w[2])),
            "mod" => Instr::Mod(Val::from_str(w[1]), Val::from_str(w[2])),
            "rcv" => Instr::Rcv(Val::from_str(w[1])),
            "jgz" => Instr::Jgz(Val::from_str(w[1]),Val::from_str(w[2])),
            _ => panic!()
        });
    }

    res
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    // #[test]
    // fn test1()
    // {
    //     assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 4);
    // }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 3);
    }
}
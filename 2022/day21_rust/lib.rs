use std::{collections::HashMap, fs};

pub type InputType = HashMap<String, Operation>;

pub enum Operation {
    Number(i64),
    Plus(String, String),
    Times(String, String),
    Minus(String, String),
    Divides(String, String),
}

#[derive(Clone, Copy)]
enum Ops {
    Val,
    Variable,
    Plus,
    Minus,
    Times,
    Divides,
}

#[derive(Clone)]
struct Formula {
    ops: Ops,
    val: Option<i64>,
    children: Vec<Formula>,
}

fn monkey_yells(name: &str, monkeys: &InputType) -> i64 {
    match monkeys.get(name).unwrap() {
        Operation::Number(x) => *x,
        Operation::Plus(s0, s1) => monkey_yells(s0, monkeys) + monkey_yells(s1, monkeys),
        Operation::Times(s0, s1) => monkey_yells(s0, monkeys) * monkey_yells(s1, monkeys),
        Operation::Minus(s0, s1) => monkey_yells(s0, monkeys) - monkey_yells(s1, monkeys),
        Operation::Divides(s0, s1) => monkey_yells(s0, monkeys) / monkey_yells(s1, monkeys),
        _ => panic!(),
    }
}

pub fn result_1(monkeys: InputType) -> i64 {
    monkey_yells("root", &monkeys)
}

fn get_formula(name: &str, monkeys: &InputType) -> Formula {
    if name == "humn" {
        Formula {
            ops: Ops::Variable,
            val: None,
            children: vec![],
        }
    } else {
        match monkeys.get(name).unwrap() {
            Operation::Number(x) => Formula {
                val: Some(*x),
                ops: Ops::Val,
                children: vec![],
            },
            Operation::Plus(s0, s1) => Formula {
                val: None,
                ops: Ops::Plus,
                children: Vec::from([get_formula(s0, monkeys), get_formula(s1, monkeys)]),
            },
            Operation::Minus(s0, s1) => Formula {
                val: None,
                ops: Ops::Minus,
                children: Vec::from([get_formula(s0, monkeys), get_formula(s1, monkeys)]),
            },
            Operation::Times(s0, s1) => Formula {
                val: None,
                ops: Ops::Times,
                children: Vec::from([get_formula(s0, monkeys), get_formula(s1, monkeys)]),
            },
            Operation::Divides(s0, s1) => Formula {
                val: None,
                ops: Ops::Divides,
                children: Vec::from([get_formula(s0, monkeys), get_formula(s1, monkeys)]),
            },
            _ => panic!(),
        }
    }
}

impl Formula {
    fn contains_variable(&self) -> bool {
        match self.ops {
            Ops::Val => false,
            Ops::Variable => true,
            _ => self.children[0].contains_variable() || self.children[1].contains_variable(),
        }
    }

    fn eval(&self) -> i64 {
        match self.ops {
            Ops::Val => self.val.unwrap(),
            Ops::Variable => panic!(),
            Ops::Plus => self.children[0].eval() + self.children[1].eval(),
            Ops::Minus => self.children[0].eval() - self.children[1].eval(),
            Ops::Times => self.children[0].eval() * self.children[1].eval(),
            Ops::Divides => self.children[0].eval() / self.children[1].eval(),
        }
    }
}

pub fn result_2(monkeys: InputType) -> i64 {
    let rs0;
    let rs1;

    match monkeys.get("root").unwrap() {
        Operation::Number(_) => panic!(),
        Operation::Plus(s0, s1) => (rs0, rs1) = (s0, s1),
        Operation::Minus(s0, s1) => (rs0, rs1) = (s0, s1),
        Operation::Times(s0, s1) => (rs0, rs1) = (s0, s1),
        Operation::Divides(s0, s1) => (rs0, rs1) = (s0, s1),
    }

    let mut formula0 = get_formula(rs0, &monkeys);
    let mut formula1 = get_formula(rs1, &monkeys);

    if !formula0.contains_variable() {
        (formula0, formula1) = (formula1, formula0);
    }

    let mut val = formula1.eval();

    loop {
        match formula0.ops {
            Ops::Val => panic!(),
            Ops::Variable => return val,
            Ops::Plus => {
                if formula0.children[0].contains_variable() {
                    val -= formula0.children[1].eval();
                    formula0 = formula0.children[0].clone();
                } else {
                    val -= formula0.children[0].eval();
                    formula0 = formula0.children[1].clone();
                }
            }
            Ops::Minus => {
                if formula0.children[0].contains_variable() {
                    val += formula0.children[1].eval();
                    formula0 = formula0.children[0].clone();
                } else {
                    val = formula0.children[0].eval() - val;
                    formula0 = formula0.children[1].clone();
                }
            }
            Ops::Times => {
                if formula0.children[0].contains_variable() {
                    val /= formula0.children[1].eval();
                    formula0 = formula0.children[0].clone();
                } else {
                    val /= formula0.children[0].eval();
                    formula0 = formula0.children[1].clone();
                }
            }
            Ops::Divides => {
                if formula0.children[0].contains_variable() {
                    val *= formula0.children[1].eval();
                    formula0 = formula0.children[0].clone();
                } else {
                    val = formula0.children[0].eval() / val;
                    formula0 = formula0.children[1].clone();
                }
            }
        }
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

    let get_op = |l: String| -> (String, Operation) {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        let op;

        if w.len() == 2 {
            op = Operation::Number(w[1].parse().unwrap());
        } else {
            match w[2] {
                "+" => op = Operation::Plus(w[1].to_string(), w[3].to_string()),
                "-" => op = Operation::Minus(w[1].to_string(), w[3].to_string()),
                "/" => op = Operation::Divides(w[1].to_string(), w[3].to_string()),
                "*" => op = Operation::Times(w[1].to_string(), w[3].to_string()),
                _ => panic!(),
            }
        }

        (w[0][0..w[0].len() - 1].to_string(), op)
    };

    input.into_iter().map(|l| get_op(l)).collect()
}

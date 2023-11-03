use std::collections::HashMap;

use crate::Solution;

#[derive(Debug)]
pub enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
pub enum Val {
    Num(i64),
    Register(String),
}

type InputType = Vec<(String, Operation, i64, Val, String, Val)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut reg: HashMap<String, i64> = HashMap::new();

    for (name, op, move_val, val0, condition, val1) in input {
        let x = match val0 {
            Val::Num(x) => x,
            Val::Register(n) => *reg.entry(n).or_insert(0),
        };

        let y = match val1 {
            Val::Num(x) => x,
            Val::Register(n) => *reg.entry(n).or_insert(0),
        };

        let res = match &condition[..] {
            "!=" => x != y,
            "==" => x == y,
            "<=" => x <= y,
            ">=" => x >= y,
            "<" => x < y,
            ">" => x > y,
            _ => panic!(),
        };

        if res {
            let entry = reg.entry(name).or_insert(0);
            *entry += match op {
                Operation::Inc => move_val,
                Operation::Dec => -move_val,
            };
        }
    }

    Solution::from(reg.into_iter().map(|(_, v)| v).max().unwrap())
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut reg: HashMap<String, i64> = HashMap::new();
    let mut max = 0;

    for (name, op, move_val, val0, condition, val1) in input {
        let x = match val0 {
            Val::Num(x) => x,
            Val::Register(n) => *reg.entry(n).or_insert(0),
        };

        let y = match val1 {
            Val::Num(x) => x,
            Val::Register(n) => *reg.entry(n).or_insert(0),
        };

        let res = match &condition[..] {
            "!=" => x != y,
            "==" => x == y,
            "<=" => x <= y,
            ">=" => x >= y,
            "<" => x < y,
            ">" => x > y,
            _ => panic!(),
        };

        if res {
            let entry = reg.entry(name).or_insert(0);
            *entry += match op {
                Operation::Inc => move_val,
                Operation::Dec => -move_val,
            };

            if *entry > max {
                max = *entry;
            }
        }
    }

    Solution::from(max)
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

        let reg = w[0].to_string();
        let op = match w[1] {
            "inc" => Operation::Inc,
            "dec" => Operation::Dec,
            _ => panic!(),
        };
        let val = w[2].parse::<i64>().unwrap();
        let val1 = if let Ok(x) = w[4].parse::<i64>() {
            Val::Num(x)
        } else {
            Val::Register(w[4].to_string())
        };

        let condition = w[5].to_string();

        let val2 = if let Ok(x) = w[6].parse::<i64>() {
            Val::Num(x)
        } else {
            Val::Register(w[6].to_string())
        };

        res.push((reg, op, val, val1, condition, val2));
    }

    res
}

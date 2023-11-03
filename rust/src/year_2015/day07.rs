use std::collections::HashMap;

use crate::Solution;

type InputType = HashMap<String, Input>;

#[derive(Debug, Clone)]
enum Operation {
    And,
    Or,
    LShift,
    RShift,
    Not,
    Val,
    ValVar,
}

#[derive(Debug, Clone)]
struct Input {
    op: Operation,
    val: Option<u16>,
    x: Option<String>,
    y: Option<String>,
}

fn get_result(key: String, map: &mut HashMap<String, Input>) -> u16 {
    let accessor = map.get_mut(&key).unwrap();

    let mut n0: Option<u16> = None;
    let mut n1: Option<u16> = None;

    if accessor.x.is_some() {
        if let Ok(n) = accessor.x.clone().unwrap().parse::<u16>() {
            n0 = Some(n);
        } else {
            let key = accessor.x.clone().unwrap();
            n0 = Some(get_result(key, map));
        }
    }

    let accessor = map.get_mut(&key).unwrap();

    if accessor.y.is_some() {
        if let Ok(n) = accessor.y.clone().unwrap().parse::<u16>() {
            n1 = Some(n);
        } else {
            let key = accessor.y.clone().unwrap();
            n1 = Some(get_result(key, map));
        }
    }

    let accessor = map.get_mut(&key).unwrap();

    let res = match accessor.op {
        Operation::Val => accessor.val.unwrap(),
        Operation::ValVar => n0.unwrap(),
        Operation::And => n0.unwrap() & n1.unwrap(),
        Operation::Or => n0.unwrap() | n1.unwrap(),
        Operation::Not => !n0.unwrap(),
        Operation::LShift => n0.unwrap() << n1.unwrap(),
        Operation::RShift => n0.unwrap() >> n1.unwrap(),
    };

    accessor.op = Operation::Val;
    accessor.val = Some(res);
    accessor.x = None;
    accessor.y = None;

    res
}

pub fn part1(s: String) -> Solution {
    let mut map = parse(s);

    Solution::from(get_result("a".to_string(), &mut map) as i64)
}

pub fn part2(s: String) -> Solution {
    let mut map = parse(s);

    let mut new_map = map.clone();
    let val = get_result("a".to_string(), &mut map);
    let accessor = new_map.get_mut(&"b".to_string()).unwrap();
    accessor.op = Operation::Val;
    accessor.val = Some(val);
    accessor.x = None;
    accessor.y = None;

    Solution::from(get_result("a".to_string(), &mut new_map) as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut map: HashMap<String, Input> = HashMap::new();

    for l in input {
        let words: Vec<&str> = l.split_whitespace().collect();
        if words.len() == 3 {
            if words[0].chars().nth(0).unwrap().is_digit(10) {
                let val = words[0].parse::<u16>().unwrap();
                map.insert(
                    words.last().unwrap().to_string(),
                    Input {
                        op: Operation::Val,
                        val: Some(val),
                        x: None,
                        y: None,
                    },
                );
            } else {
                let x = Some(words[0].to_string());
                map.insert(
                    words.last().unwrap().to_string(),
                    Input {
                        op: Operation::ValVar,
                        val: None,
                        x,
                        y: None,
                    },
                );
            }
        } else if words.len() == 4 {
            let x = Some(words[1].to_string());
            map.insert(
                words.last().unwrap().to_string(),
                Input {
                    op: Operation::Not,
                    val: None,
                    x,
                    y: None,
                },
            );
        } else {
            let x = Some(words[0].to_string());
            let y = Some(words[2].to_string());
            let op = match words[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift,
                "RSHIFT" => Operation::RShift,
                _ => Operation::LShift,
            };

            map.insert(
                words.last().unwrap().to_string(),
                Input {
                    op,
                    val: None,
                    x,
                    y,
                },
            );
        }
    }

    map
}

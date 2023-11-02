use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<(i64, i64, i64, i64, i64, i64)>;

// 0 => value goes to bot (0,bot,value,0,0,0)
// 1 => low goes to, high goes to (1,bot,destination_low,destination_high,is_bot,is_bot)
// -1 => is output, 1 => is bot
pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    let mut outputs: HashMap<i64, i64> = HashMap::new();
    let mut bots: HashMap<i64, Vec<i64>> = HashMap::new();

    while !input.is_empty() {
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..input.len() {
            let (instr, bot, n0, n1, is_bot0, is_bot1) = input[i];

            if instr == 0 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    accessor.push(n0);
                    to_remove.push(i);
                }
            } else if instr == 1 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    continue;
                }
                accessor.sort();
                let (x, y) = (accessor[0], accessor[1]);

                if x == 17 && y == 61 {
                    return Solution::from(bot);
                }

                if is_bot0 == -1 {
                    let accessor1 = outputs.entry(n0).or_insert(0);
                    *accessor1 = x;
                } else {
                    let accessor1 = bots.entry(n0).or_insert(vec![]);
                    accessor1.push(x);
                }

                if is_bot1 == -1 {
                    let accessor1 = outputs.entry(n1).or_insert(0);
                    *accessor1 = y;
                } else {
                    let accessor1 = bots.entry(n1).or_insert(vec![]);
                    accessor1.push(y)
                }
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            input.remove(i);
        }
    }

    Solution::from(-1)
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    let mut outputs: HashMap<i64, i64> = HashMap::new();
    let mut bots: HashMap<i64, Vec<i64>> = HashMap::new();

    while !input.is_empty() {
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..input.len() {
            let (instr, bot, n0, n1, is_bot0, is_bot1) = input[i];

            if instr == 0 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    accessor.push(n0);
                    to_remove.push(i);
                }
            } else if instr == 1 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    continue;
                }
                accessor.sort();
                let (x, y) = (accessor[0], accessor[1]);

                if is_bot0 == -1 {
                    let accessor1 = outputs.entry(n0).or_insert(0);
                    *accessor1 = x;
                } else {
                    let accessor1 = bots.entry(n0).or_insert(vec![]);
                    accessor1.push(x);
                }

                if is_bot1 == -1 {
                    let accessor1 = outputs.entry(n1).or_insert(0);
                    *accessor1 = y;
                } else {
                    let accessor1 = bots.entry(n1).or_insert(vec![]);
                    accessor1.push(y)
                }
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            input.remove(i);
        }
    }

    if let Some(m) = outputs.get(&0) {
        if let Some(n) = outputs.get(&1) {
            if let Some(p) = outputs.get(&2) {
                return Solution::from(m * n * p);
            }
        }
    }

    Solution::from(-1)
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
        match w[0] {
            "value" => {
                res.push((
                    0,
                    w.last().unwrap().parse().unwrap(),
                    w[1].parse().unwrap(),
                    0,
                    0,
                    0,
                ));
            }
            _ => {
                let is_bot0 = if w[5] == "bot" { 1 } else { -1 };
                let is_bot1 = if w[10] == "bot" { 1 } else { -1 };
                res.push((
                    1,
                    w[1].parse().unwrap(),
                    w[6].parse::<i64>().unwrap(),
                    w.last().unwrap().parse::<i64>().unwrap(),
                    is_bot0,
                    is_bot1,
                ));
            }
        }
    }

    res
}

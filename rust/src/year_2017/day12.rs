use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<(u64, Vec<u64>)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut seen: HashMap<u64, bool> = HashMap::new();
    let mut to_explore: Vec<u64> = vec![];
    to_explore.push(0);

    while !to_explore.is_empty() {
        let current = to_explore.pop().unwrap();
        seen.insert(current, true);
        for other in input.get(current as usize).unwrap().1.iter() {
            if !seen.contains_key(other) && !to_explore.contains(other) {
                to_explore.push(*other);
            }
        }
    }

    Solution::from(seen.len() as i64)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut seen: HashMap<u64, bool> = HashMap::new();
    let mut to_explore: Vec<u64> = vec![];

    // entry point for new group discovery
    let mut current_explore_index = 0;
    let mut groups_count = 0;

    loop {
        if to_explore.is_empty() {
            while seen.contains_key(&current_explore_index) {
                current_explore_index += 1;
            }

            if current_explore_index >= input.len() as u64 {
                break;
            }

            to_explore.push(current_explore_index as u64);
            groups_count += 1;
        }

        let current = to_explore.pop().unwrap();
        seen.insert(current, true);
        for other in input.get(current as usize).unwrap().1.iter() {
            if !seen.contains_key(other) && !to_explore.contains(other) {
                to_explore.push(*other);
            }
        }
    }

    Solution::from(groups_count as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for mut line in input {
        line = line.replace(",", " ");
        let w = line.split_whitespace().collect::<Vec<&str>>();
        res.push((
            w[0].parse().unwrap(),
            w[2..].iter().map(|el| el.parse().unwrap()).collect(),
        ));
    }

    res
}

use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<Vec<i32>>;

pub fn part1(s: String) -> Solution {
    let risks = parse(s);

    let width = risks.len() as i32;
    let height = risks[0].len() as i32;

    let mut explored_cases: HashMap<(i32, i32), i32> = HashMap::new();
    let mut to_explore_cases: HashMap<(i32, i32), i32> = HashMap::new();

    to_explore_cases.insert((0, 0), 0);

    while !to_explore_cases.is_empty() {
        let (&case, &cost) = to_explore_cases
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        if case == (width - 1, height - 1) {
            return Solution::from(cost as i32);
        }

        to_explore_cases.remove(&case);

        explored_cases.insert(case, cost);

        for (x, y) in [
            (case.0 - 1, case.1),
            (case.0 + 1, case.1),
            (case.0, case.1 - 1),
            (case.0, case.1 + 1),
        ] {
            if x < 0 || y < 0 || x >= width || y >= height {
                continue;
            }
            if explored_cases.contains_key(&(x, y)) {
                continue;
            }

            let access = to_explore_cases.entry((x, y)).or_insert(i32::MAX);
            let new_possible_cost = cost + risks[x as usize][y as usize];
            if new_possible_cost < *access {
                *access = new_possible_cost;
            }
        }
    }

    Solution::NotFound
}

fn risk_calculus(x: &i32, y: &i32, width: &i32, height: &i32, risks: &Vec<Vec<i32>>) -> i32 {
    let addition: i32 = x / width + y / height;
    let mut candidate = risks[(x % width) as usize][(y % height) as usize] + addition;
    while candidate > 9 {
        candidate -= 9;
    }

    candidate
}

pub fn part2(s: String) -> Solution {
    let risks = parse(s);

    let width = risks.len() as i32;
    let height = risks[0].len() as i32;

    let mut explored_cases: HashMap<(i32, i32), i32> = HashMap::new();
    let mut to_explore_cases: HashMap<(i32, i32), i32> = HashMap::new();

    to_explore_cases.insert((0, 0), 0);

    while !to_explore_cases.is_empty() {
        let (&case, &cost) = to_explore_cases
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        if case == (5 * width - 1, 5 * height - 1) {
            return Solution::from(cost as i32);
        }

        to_explore_cases.remove(&case);

        explored_cases.insert(case, cost);

        for (x, y) in [
            (case.0 - 1, case.1),
            (case.0 + 1, case.1),
            (case.0, case.1 - 1),
            (case.0, case.1 + 1),
        ] {
            if x < 0 || y < 0 || x >= (5 * width) || y >= (5 * height) {
                continue;
            }
            if explored_cases.contains_key(&(x, y)) {
                continue;
            }

            let access = to_explore_cases.entry((x, y)).or_insert(i32::MAX);
            let new_possible_cost = cost + risk_calculus(&x, &y, &width, &height, &risks);
            if new_possible_cost < *access {
                *access = new_possible_cost;
            }
        }
    }

    Solution::NotFound
}

fn parse(s: String) -> InputType {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    input
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

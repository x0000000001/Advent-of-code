use crate::Solution;

type InputType = Vec<i64>;

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    let mut index: i64 = 0;
    let mut count = 0;

    while index >= 0 && index < input.len() as i64 {
        input[index as usize] += 1;
        index += input[index as usize] - 1;
        count += 1;
    }

    Solution::from(count)
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    let mut index: i64 = 0;
    let mut count = 0;

    while index >= 0 && index < input.len() as i64 {
        let step_jump = input[index as usize];
        input[index as usize] += if step_jump >= 3 { -1 } else { 1 };
        index += step_jump;
        count += 1;
    }

    Solution::from(count)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .map(|el| el.parse().unwrap())
        .collect()
}

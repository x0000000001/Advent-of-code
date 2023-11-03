use crate::Solution;

type InputType = Vec<u8>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut sum: i64 = 0;

    for i in 0..input.len() {
        if input[i] == input[(i + 1) % input.len()] {
            sum += input[i] as i64;
        }
    }

    Solution::from(sum)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut sum: i64 = 0;
    let len = input.len();

    for i in 0..len {
        if input[i] == input[(i + len / 2) % len] {
            sum += input[i] as i64;
        }
    }

    Solution::from(sum)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

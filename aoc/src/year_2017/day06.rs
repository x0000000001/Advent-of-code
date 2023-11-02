use crate::Solution;

type InputType = Vec<u64>;

fn transition(input: &mut InputType) {
    // reversing to respect the "first wins" rule
    let max_index = input.len()
        - 1
        - input
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|(_, x)| *x)
            .unwrap()
            .0;
    let mut stock = input[max_index];
    input[max_index] = 0;
    let mut give_index = (max_index + 1) % input.len();

    while stock > 0 {
        input[give_index] += 1;
        give_index = (give_index + 1) % input.len();
        stock -= 1;
    }
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    let mut seen: Vec<InputType> = vec![];
    let mut count = 0;

    while !seen.contains(&input) {
        seen.push(input.clone());
        transition(&mut input);
        count += 1;
    }

    Solution::from(count)
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    let mut seen: Vec<InputType> = vec![];
    let mut count = 0;

    while !seen.contains(&input) {
        seen.push(input.clone());
        transition(&mut input);
        count += 1;
    }

    Solution::from(count - seen.into_iter().position(|x| x == input).unwrap() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .split_whitespace()
        .map(|el| el.parse().unwrap())
        .collect()
}

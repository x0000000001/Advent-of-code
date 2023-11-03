use crate::Solution;

type InputType = (usize, usize);

fn loop_size(key: usize, subject: usize) -> usize {
    let mut r = 1;
    let mut i = 0;

    while r != key {
        r *= subject;
        r %= 20201227;
        i += 1;
    }

    i
}

fn produce_key(subject: usize, loop_size: usize) -> usize {
    let mut r = 1;

    for _ in 0..loop_size {
        r *= subject;
        r %= 20201227;
    }

    return r;
}

pub fn part1(s: String) -> Solution {
    let (cardpb, doorpb) = parse(s);

    Solution::from(produce_key(doorpb, loop_size(cardpb, 7)) as i64)
}

pub fn part2(_: String) -> Solution {
    Solution::Day25Part2
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    (input[0].parse().unwrap(), input[1].parse().unwrap())
}

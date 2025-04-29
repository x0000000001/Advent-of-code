use std::{collections::HashMap, fs};

type InputType = (Vec<usize>, Vec<usize>);

pub fn part1(s: String) {
    let (mut left, mut right) = parse(s);
    left.sort();
right.sort();

    let result: usize = (0..left.len()).map(|i| left[i].abs_diff(right[i])).sum();
    println!("{result}");
}

pub fn part2(s: String) {
    let (left, right) = parse(s);
    let mut set: HashMap<usize, usize> = HashMap::new();

    right.into_iter().for_each(|n| {
        *(set.entry(n).or_insert(0)) += 1;
    });

    let result: usize = left
        .into_iter()
        .map(|n| *set.get(&n).unwrap_or(&0) * n)
        .sum();
    println!("{result}");
}

fn parse(s: String) -> InputType {
    let vec: Vec<(usize, usize)> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums: Vec<usize> = l.split_whitespace().map(|w| w.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let left: Vec<usize> = vec.iter().map(|(left, _)| *left).collect();
    let right: Vec<usize> = vec.iter().map(|(_, right)| *right).collect();

    (left, right)
}

fn main() {
    let s = fs::read_to_string("data/input_example.txt").unwrap();
    part1(s.clone());
    part2(s.clone());
    let s = fs::read_to_string("data/input.txt").unwrap();
    part1(s.clone());
    part2(s.clone());
}

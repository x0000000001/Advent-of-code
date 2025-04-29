use std::{collections::HashMap, fs};

type InputType = Vec<String>;

pub fn part1(s: String) {}

pub fn part2(s: String) {}

fn parse(s: String) -> InputType {
    s.lines().map(|str| str.to_string()).collect()
}

fn main() {
    let s = fs::read_to_string("data/input_example.txt").unwrap();
    part1(s.clone());
    part2(s.clone());
    let s = fs::read_to_string("data/input.txt").unwrap();
    part1(s.clone());
    part2(s.clone());
}

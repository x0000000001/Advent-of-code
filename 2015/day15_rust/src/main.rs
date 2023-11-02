use day15::*;
use std::fs;
use std::time::Instant;

const INPUT_PATH: &str = "input.txt";

fn ex_function(foo: fn(InputType) -> i64, name: &str) {
    let now = Instant::now();
    let result = foo(read_input(INPUT_PATH)) as i64;
    println!(
        "{name} -> {result} {}, {:.2?}",
        " ".repeat(20 - result.to_string().len()),
        now.elapsed()
    );
}

fn main() {
    ex_function(result_1, "result 1");
    ex_function(result_2, "result 2");
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = Vec::new();

    for l in input {
        let l = l.replace(",", "");
        let words: Vec<&str> = l.split_whitespace().collect();
        let capacity: i64 = words[2].parse().unwrap();
        let durability: i64 = words[4].parse().unwrap();
        let flavour: i64 = words[6].parse().unwrap();
        let texture: i64 = words[8].parse().unwrap();
        let calories: i64 = words[10].parse().unwrap();

        res.push([capacity, durability, flavour, texture, calories])
    }

    res
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

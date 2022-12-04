use std::fs;

pub type InputType = Vec<i64>;

const base_pattern: [i64; 4] = [0, 1, 0, -1];

fn get_pattern(i: usize, j: usize) -> i64 {
    let index = (j + 1) / (i + 1);
    base_pattern[index % 4]
}

fn mutate(input: &mut InputType) {
    let mut result = vec![0; input.len()];

    for i in 0..result.len() {
        result[i] = input
            .iter()
            .enumerate()
            .map(|(j, v)| get_pattern(i, j) * v)
            .sum();

        result[i] = (result[i] % 10).abs();
    }

    for i in 0..result.len() {
        input[i] = result[i];
    }
}

pub fn result_1(mut input: InputType) -> i64 {
    for _ in 0..100 {
        mutate(&mut input);
    }

    println!(
        "{}",
        input[0..8]
            .into_iter()
            .fold("".to_string(), |acc, b| format!("{}{}", acc, b.to_string()))
    );

    0
}

pub fn result_2(input: InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input[0]
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

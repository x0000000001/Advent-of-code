use std::fs;

pub type InputType = (usize, usize);

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

pub fn result_1((cardpb, doorpb): InputType) -> i64 {
    produce_key(doorpb, loop_size(cardpb, 7)) as i64
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

    (input[0].parse().unwrap(), input[1].parse().unwrap())
}

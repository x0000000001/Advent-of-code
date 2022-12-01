use std::fs;

const FILE: &str = "input.txt";

pub fn result_1() -> i32 {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let positions: Vec<i32> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    let mut min = std::i32::MAX;

    let max_pos = positions.iter().max().unwrap();

    for i in 0..*max_pos {
        let cost: i32 = positions.iter().map(|el| (el - i).abs()).sum();

        if cost < min {
            min = cost;
        }
    }

    min
}

pub fn result_2() -> i32 {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let positions: Vec<i32> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    let mut min = std::i32::MAX;

    let max_pos = positions.iter().max().unwrap();

    for i in 0..*max_pos {
        let cost: i32 = positions
            .iter()
            .map(|el| ((el - i).abs() * ((el - i).abs() + 1)) / 2)
            .sum();

        if cost < min {
            min = cost;
        }
    }

    min
}

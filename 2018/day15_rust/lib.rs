use std::fs;

pub enum Case {
    Elf,
    Goblin,
    Wall,
    Empty,
}

pub type InputType = Vec<Vec<Case>>;

fn get_adjacent(input: &InputType, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if x < input.len() - 1 {
        result.push((x + 1, y));
    }
    if y < input[0].len() - 1 {
        result.push((x, y + 1));
    }
    result
}

pub fn result_1(input: InputType) -> i64 {
    0
}

pub fn result_2(input: InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    'E' => Case::Elf,
                    'G' => Case::Goblin,
                    '#' => Case::Wall,
                    '.' => Case::Empty,
                    _ => panic!("Unknown case"),
                })
                .collect()
        })
        .collect()
}

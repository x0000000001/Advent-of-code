use std::{collections::HashSet, fs};

// (write 1, go right, index of next state) * 2
type State = ((bool, bool, usize), (bool, bool, usize));

pub type InputType = (usize, Vec<State>);

pub fn result_1((steps, states): InputType) -> i64 {
    let mut pos = 0;
    let mut state = 0;
    let mut s = HashSet::new();

    for _ in 0..steps {
        let is_1 = s.contains(&pos);

        let rules_to_follow = if is_1 {
            states[state].1
        } else {
            states[state].0
        };

        if rules_to_follow.0 && !is_1 {
            s.insert(pos);
        } else if !rules_to_follow.0 && is_1 {
            s.remove(&pos);
        }

        pos += if rules_to_follow.1 { 1 } else { -1 };
        state = rules_to_follow.2;
    }

    s.len() as i64
}

pub fn result_2(input: InputType) -> i64 {
    0
}

fn retrieve_orders(input: &Vec<String>) -> (bool, bool, usize) {
    (
        input[0].split_whitespace().last().unwrap().replace(".", "") == "1",
        input[1].split_whitespace().last().unwrap().replace(".", "") == "right",
        match input[2]
            .split_whitespace()
            .last()
            .unwrap()
            .replace(".", "")
            .as_str()
        {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            "D" => 3,
            "E" => 4,
            "F" => 5,
            "G" => 6,
            "H" => 7,
            _ => panic!(),
        },
    )
}

fn retrieve_state(input: &Vec<String>) -> State {
    (
        retrieve_orders(&input[2..5].to_vec()),
        retrieve_orders(&input[6..9].to_vec()),
    )
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let steps = input[1].split_whitespace().collect::<Vec<&str>>()[5]
        .parse()
        .unwrap();

    input.remove(0);
    input.remove(1);

    (
        steps,
        (0..input.len() / 9)
            .map(|i| retrieve_state(&input[9 * i..(9 * (i + 1))].to_vec()))
            .collect(),
    )
}

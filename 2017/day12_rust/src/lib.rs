use std::{collections::HashMap, fs};

pub type InputType = Vec<(u64, Vec<u64>)>;

pub fn result_1(input: InputType) -> i64 {
    let mut seen: HashMap<u64, bool> = HashMap::new();
    let mut to_explore: Vec<u64> = vec![];
    to_explore.push(0);

    while !to_explore.is_empty() {
        let current = to_explore.pop().unwrap();
        seen.insert(current, true);
        for other in input.get(current as usize).unwrap().1.iter() {
            if !seen.contains_key(other) && !to_explore.contains(other) {
                to_explore.push(*other);
            }
        }
    }

    seen.len() as i64;
}

pub fn result_2(input: InputType) -> i64 {
    let mut seen: HashMap<u64, bool> = HashMap::new();
    let mut to_explore: Vec<u64> = vec![];

    // entry point for new group discovery
    let mut current_explore_index = 0;
    let mut groups_count = 0;

    loop {
        if to_explore.is_empty() {
            while seen.contains_key(&current_explore_index) {
                current_explore_index += 1;
            }

            if current_explore_index >= input.len() as u64 {
                break;
            }

            to_explore.push(current_explore_index as u64);
            groups_count += 1;
        }

        let current = to_explore.pop().unwrap();
        seen.insert(current, true);
        for other in input.get(current as usize).unwrap().1.iter() {
            if !seen.contains_key(other) && !to_explore.contains(other) {
                to_explore.push(*other);
            }
        }
    }

    groups_count as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for mut line in input {
        line = line.replace(",", " ");
        let w = line.split_whitespace().collect::<Vec<&str>>();
        res.push((
            w[0].parse().unwrap(),
            w[2..].iter().map(|el| el.parse().unwrap()).collect(),
        ));
    }

    res
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 2);
    }
}

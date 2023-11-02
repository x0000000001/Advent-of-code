use std::fs;

pub type InputType = Vec<i64>;

pub fn result_1(mut input: InputType) -> i64 {
    let mut index: i64 = 0;
    let mut count = 0;

    while index >= 0 && index < input.len() as i64 {
        input[index as usize] += 1;
        index += input[index as usize] - 1;
        count += 1;
    }

    count
}

pub fn result_2(mut input: InputType) -> i64 {
    let mut index: i64 = 0;
    let mut count = 0;

    while index >= 0 && index < input.len() as i64 {
        let step_jump = input[index as usize];
        input[index as usize] += if step_jump >= 3 { -1 } else { 1 };
        index += step_jump;
        count += 1;
    }
    count
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input.into_iter().map(|el| el.parse().unwrap()).collect()
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 0);
    }
}

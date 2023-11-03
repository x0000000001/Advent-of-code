use day17::*;
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

    input
        .iter()
        .map(|el| el.parse().unwrap())
        .collect::<Vec<u64>>()
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_ways() {
        assert_eq!(count_ways(25, &Vec::from([20, 15, 10, 5, 5])), 4);
    }

    #[test]
    fn test_count_min_ways() {
        assert_eq!(count_min_ways(25, &Vec::from([20, 15, 10, 5, 5])), (2, 3));
    }
}

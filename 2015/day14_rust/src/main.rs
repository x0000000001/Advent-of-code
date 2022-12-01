use day14::*;
use std::collections::HashMap;
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

    let mut res: InputType = HashMap::new();

    for l in input {
        let words: Vec<&str> = l.split_whitespace().collect();
        let speed: i64 = words[3].parse().unwrap();
        let fly_time: i64 = words[6].parse().unwrap();
        let rest_time: i64 = words[13].parse().unwrap();

        res.insert(
            words[0].to_string(),
            Reindeer::create(speed, fly_time, rest_time),
        );
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
        let mut r0 = Reindeer::create(14, 10, 127);
        let mut r1 = Reindeer::create(16, 11, 162);
        r0.step();
        r1.step();
        assert_eq!(r0.get_score(), 14);
        assert_eq!(r1.get_score(), 16);

        let mut r0 = Reindeer::create(14, 10, 127);
        let mut r1 = Reindeer::create(16, 11, 162);
        for _ in 0..1000 {
            r0.step();
            r1.step();
        }
        assert_eq!(r0.get_score(), 1120);
        assert_eq!(r1.get_score(), 1056);
    }

    #[test]
    fn test2() {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}

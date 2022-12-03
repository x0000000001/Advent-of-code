use day14_rust::*;
use std::time::Instant;

const INPUT_PATH: &str = "input.txt";
const TEST_PATH: &str = "test_input.txt";

fn ex_function(foo: fn(InputType) -> i64, name: &str, path: &str) {
    let now = Instant::now();
    let result = foo(read_input(path)) as i64;
    println!(
        "{name} -> {result} {}, {:.2?}",
        " ".repeat(20 - result.to_string().len()),
        now.elapsed()
    );
}

// 432529 too high
fn main() {
    ex_function(result_1, "test 1.0", TEST_PATH);
    ex_function(result_1, "test 1.1", "test_input1.txt");
    ex_function(result_1, "test 1.2", "test_input2.txt");
    ex_function(result_2, "test 2.0", TEST_PATH);
    ex_function(result_2, "test 2.1", "test_input1.txt");
    ex_function(result_2, "test 2.2", "test_input2.txt");
    ex_function(result_1, "result 1", INPUT_PATH);
    ex_function(result_2, "result 2", INPUT_PATH);
}
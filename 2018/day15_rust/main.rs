use day15_rust::*;
use std::time::Instant;

const INPUT_PATH: &str = "input.txt";
const TEST_PATH: &str = "test_input.txt";

fn ex_function(foo: fn(Map) -> i64, name: &str, path: &str) {
    let now = Instant::now();
    let result = foo(read_input(path)) as i64;
    println!(
        "{name} -> {result} {}, {:.2?}",
        " ".repeat(20 - result.to_string().len()),
        now.elapsed()
    );
}

fn main() {
    ex_function(result_1, "test 1.0 (27730)", TEST_PATH);
    ex_function(result_1, "test 1.1 (36334)", "test_input1.txt");
    ex_function(result_1, "test 1.2 (39514)", "test_input2.txt");
    ex_function(result_1, "test 1.3 (27755)", "test_input3.txt");
    ex_function(result_1, "test 1.4 (28944)", "test_input4.txt");
    ex_function(result_1, "test 1.5 (18740)", "test_input5.txt");
    ex_function(result_2, "test 2.0 (4988)", "test_input.txt");
    ex_function(result_2, "test 2.2 (31284)", "test_input2.txt");
    ex_function(result_2, "test 2.3 (3478)", "test_input3.txt");
    ex_function(result_2, "test 2.4 (6474)", "test_input4.txt");
    ex_function(result_2, "test 2.5 (1140)", "test_input5.txt");
    ex_function(result_1, "result 1", INPUT_PATH);
    ex_function(result_2, "result 2", INPUT_PATH);
}

use day18_rust::*;
use std::time::Instant;

const INPUT_PATH: &str = "input.txt";

fn ex_function(foo: fn(InputType) -> i64, name: &str, path: &str, expected_result: Option<i64>) {
    let now = Instant::now();
    let result = foo(read_input(path)) as i64;

    if let Some(x) = expected_result {
        assert_eq!(
            x, result,
            "test {} failed : {} (result) != {} (expected)",
            name, x, result
        );
    }

    println!(
        "{name} -> {result} {}, {:.2?}",
        " ".repeat(20 - result.to_string().len()),
        now.elapsed()
    );
}

fn main() {
    ex_function(result_1, "test 1.0", "test_input0.txt", Some(8));
    ex_function(result_1, "test 1.1", "test_input1.txt", Some(86));
    ex_function(result_1, "test 1.2", "test_input2.txt", Some(132));
    // ex_function(result_1, "test 1.3", "test_input3.txt", Some(136));
    ex_function(result_1, "result 1", INPUT_PATH, None);
    // ex_function(result_2, "result 2", INPUT_PATH, None);
}

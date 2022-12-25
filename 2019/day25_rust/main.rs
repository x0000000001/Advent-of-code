use day25_rust::*;
use std::time::Instant;

const INPUT_PATH: &str = "input.txt";

fn ex_function(foo: fn(InputType) -> i64, name: &str, path: &str) {
    let now = Instant::now();
    let result = foo(read_input(path)) as i64;
    println!(
        "{name} -> {result} {}, {:.2?}",
        " ".repeat(20 - result.to_string().len()),
        now.elapsed()
    );
}

fn main() {
    ex_function(result_1, "result 1", INPUT_PATH);
    ex_function(result_2, "result 2", INPUT_PATH);
}

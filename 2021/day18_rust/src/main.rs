use day18::{result_1, result_2};
use std::time::Instant;

fn ex_function(foo: fn() -> i64, name: &str) {
    let now = Instant::now();
    let result = foo();
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

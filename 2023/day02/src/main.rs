use std::time;

use dayrust::{part1, part2};

fn exec_func<F>(f: F, input: String, name: &str)
where
    F: Fn(String) -> dayrust::Solution,
{
    let t_start = time::Instant::now();
    let sol = f(input);
    let time = time::Instant::now() - t_start;
    println!("{:<15} : {:<40} ({:<15?})", name, sol.to_string(), time);
}

fn main() {
    let s_test = String::from(include_str!("../test_input0.txt"));
    let s = String::from(include_str!("../input.txt"));
    exec_func(part1, s_test.clone(), "PART 1 TEST");
    exec_func(part1, s.clone(), "PART 1");
    exec_func(part2, s_test.clone(), "PART 2 TEST");
    exec_func(part2, s.clone(), "PART 2");
}

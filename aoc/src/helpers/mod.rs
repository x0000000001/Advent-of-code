use std::time::{Duration, Instant};

mod aoc_day;
mod path;
mod solution;

pub use aoc_day::{AocDay, AocImplementation, OutputType};
pub use solution::Solution;

/// Type of an aoc solving function.
///
/// # Input
///
/// Path to an input (String)
///
/// # Output
///
/// Solution (Solution)
pub type SolFuncType = fn(String) -> Solution;

fn ex_function(solve_func: SolFuncType, input_str: String) -> (Solution, Duration) {
    let now = Instant::now();
    let result = solve_func(input_str);
    (result, now.elapsed())
}

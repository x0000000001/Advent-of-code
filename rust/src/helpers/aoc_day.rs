use std::fmt::Display;
use std::time::Duration;

use super::ex_function;
use super::path::{get_input_paths, get_path};
use super::solution::Solution;
use super::SolFuncType;

/// Wrapper to represent an aoc day solution.
#[derive(Clone)]
pub enum AocImplementation {
    /// Day implemented in Rust, containing part 1 and part 2 functions.
    Some(SolFuncType, SolFuncType),
    /// Contains an option for the language name.
    ImplementedInOtherLanguage(Option<&'static str>),
    NotImplementedYet,
}

impl Display for AocImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocImplementation::Some(_, _) => {
                write!(f, "")
            }
            AocImplementation::ImplementedInOtherLanguage(Some(name)) => {
                write!(f, "Implemented in {name}.")
            }
            AocImplementation::ImplementedInOtherLanguage(None) => {
                write!(f, "Implemented in another unknown language.")
            }
            AocImplementation::NotImplementedYet => write!(f, "Not implemented yet."),
        }
    }
}

pub struct AocDay {
    pub name: &'static str,
    pub implementation: AocImplementation,
}

/// Describes how output should be displayed after solving an aoc problem.
pub enum OutputType {
    /// Show output for both parts with execution time.
    Full,
    /// Show only execution time, one line for both output.
    OneLineTime,
    None,
}

pub enum DayExecutionResult {
    Success((Solution, Duration), (Solution, Duration)),
    ImplementationError(AocImplementation),
    FileReadingError(String),
}

impl AocDay {
    pub fn test_input(&self, part: usize, test_id: usize, expected_result: Solution) {
        if let AocImplementation::Some(part1, part2) = self.implementation {
            let path = get_path(self.name, Some(test_id));

            let input_string = match std::fs::read_to_string(path.clone()) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Problem reading file {path} for test input {test_id} : {err}");
                }
            };

            let (result, _) = ex_function(
                match part {
                    1 => part1,
                    2 => part2,
                    x => panic!("Part {} ? There's no such part in an aoc problem !", x),
                },
                input_string,
            );
            assert_eq!(
                result, expected_result,
                "Wrong result : expected {}, got {}",
                expected_result, result
            );
        } else {
            panic!("Problem testing {} : {}", self.name, self.implementation);
        }
    }

    pub fn solve_path(&self, path: &str) -> DayExecutionResult {
        let input_string = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(err) => {
                return DayExecutionResult::FileReadingError(format!("{err}"));
            }
        };

        match self.implementation {
            AocImplementation::Some(part1, part2) => DayExecutionResult::Success(
                ex_function(part1, input_string.clone()),
                ex_function(part2, input_string),
            ),
            AocImplementation::ImplementedInOtherLanguage(_)
            | AocImplementation::NotImplementedYet => {
                DayExecutionResult::ImplementationError(self.implementation.clone())
            }
        }
    }

    pub fn solve(&self) -> DayExecutionResult {
        let path = get_path(self.name, None);
        self.solve_path(&path)
    }

    pub fn solve_test(&self, test_id: usize) -> DayExecutionResult {
        let path = get_path(self.name, Some(test_id));
        self.solve_path(&path)
    }

    pub fn solve_all_tests(&self) -> Vec<DayExecutionResult> {
        get_input_paths(self.name)
            .into_iter()
            .map(|p| self.solve_path(&p))
            .collect()
    }
}

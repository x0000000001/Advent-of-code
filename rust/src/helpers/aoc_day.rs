use std::fmt::Display;
use std::time::Duration;

use super::ex_function;
use super::input::Input;
use super::solution::Solution;
use super::SolFuncType;

/// Wrapper to represent an aoc day solution.
#[derive(Hash, Clone)]
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
            AocImplementation::Some(_, _) => "".fmt(f),
            AocImplementation::ImplementedInOtherLanguage(language) => format!(
                "Implemented in {}.",
                if let Some(name) = language { name } else { "?" }
            )
            .fmt(f),
            AocImplementation::NotImplementedYet => "Not implemented yet.".fmt(f),
        }
    }
}

#[derive(Hash, Clone)]
pub struct AocDay {
    pub year: usize,
    pub day: usize,
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
    pub fn test_input(&self, part: usize, input: &Input, expected_result: Solution) {
        if let AocImplementation::Some(part1, part2) = self.implementation {
            let path = input.path(self);

            let input_string = match std::fs::read_to_string(path.clone()) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Problem reading file {path} : {err}",);
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
            panic!(
                "Problem testing day {} {}: {}",
                self.year, self.day, self.implementation
            );
        }
    }

    pub fn solve(&self, input: &Input) -> DayExecutionResult {
        let input_string = match std::fs::read_to_string(input.path(self)) {
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

    pub fn solve_tests(&self, user: &'static str) -> Vec<DayExecutionResult> {
        Input::get_all(user, self)
            .into_iter()
            .filter_map(|p| {
                if p.test_id.is_none() {
                    None
                } else {
                    Some(self.solve(&p))
                }
            })
            .collect()
    }
}

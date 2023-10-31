use std::fmt::Display;
use std::time::Duration;

use super::ex_function;
use super::path::get_path;
use super::solution::Solution;
use super::SolFuncType;

/// Wrapper to represent an aoc day solution.
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
                write!(f, "Implemented in {name}.\n")
            }
            AocImplementation::ImplementedInOtherLanguage(None) => {
                write!(f, "Implemented in another unknown language.\n")
            }
            AocImplementation::NotImplementedYet => write!(f, "Not implemented yet.\n"),
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

fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => panic!("{}", err),
    }
}

impl AocDay {
    pub fn test_input(&self, part: usize, test_id: usize, expected_result: Solution) {
        if let AocImplementation::Some(part1, part2) = self.implementation {
            let path = get_path(self.name, Some(test_id));
            let input_string = read_file(&path);
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

    fn solve_full(&self) {
        println!("--- {} ---", self.name);
        match self.implementation {
            AocImplementation::Some(part1, part2) => {
                let path = get_path(self.name, None);
                let input_string = read_file(&path);

                let print_line = |(sol, duration): (Solution, Duration), name: &str| match sol {
                    sol @ Solution::Num(_)
                    | sol @ Solution::String(_)
                    | sol @ Solution::NotFound => println!(
                        "{name} -> {sol} {}, {:.2?}",
                        " ".repeat(20 - sol.to_string().len()),
                        duration
                    ),
                    Solution::NotImplemented => println!("{}", sol),
                };

                print_line(ex_function(part1, input_string.clone()), "Part 1");
                print_line(ex_function(part2, input_string), "Part 2");
            }
            _ => (),
        }
        println!("{}", self.implementation);
    }

    fn solve_time(&self) {
        print!("{} || ", self.name);
        match self.implementation {
            AocImplementation::Some(part1, part2) => {
                let path = get_path(self.name, None);
                let input_str = read_file(&path);

                let print_time = |(sol, duration): (Solution, Duration)| {
                    let s = match sol {
                        Solution::Num(_) | Solution::String(_) | Solution::NotFound => {
                            format!("{:.2?}", duration)
                        }
                        Solution::NotImplemented => {
                            format!("{}", self.implementation)
                        }
                    };

                    print!("{}{}", s, " ".repeat(20 - s.len()))
                };

                print_time(ex_function(part1, input_str.clone()));
                print!("|| ");
                print_time(ex_function(part2, input_str));
                println!();
            }
            _ => println!("{}", self.implementation),
        };
    }

    fn solve_quiet(&self) {
        match self.implementation {
            AocImplementation::Some(part1, part2) => {
                let path = get_path(self.name, None);
                let input_str = read_file(&path);
                ex_function(part1, input_str.clone());
                ex_function(part2, input_str);
            }
            _ => panic!("{}", self.implementation),
        }
    }

    pub fn solve(&self, output_type: OutputType) {
        match output_type {
            OutputType::Full => self.solve_full(),
            OutputType::OneLineTime => self.solve_time(),
            OutputType::None => self.solve_quiet(),
        }
    }
}

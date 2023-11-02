use std::fmt::Display;
use std::time::Duration;

use colorize::global_fg;

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

fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => panic!("Error while trying to open {} : {}", path, err),
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
                    Solution::Num(_) | Solution::String(_) | Solution::NotFound => println!(
                        "{name} -> {sol} {}, {:.2?}",
                        " ".repeat(20 - sol.to_string().len()),
                        duration
                    ),
                    Solution::NotImplemented | Solution::Day25Part2 => println!("{}", sol),
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
                            format!("{:.0?}", duration)
                        }
                        Solution::NotImplemented | Solution::Day25Part2 => {
                            format!("{}", sol)
                        }
                    };

                    let color = if duration < Duration::from_millis(10) {
                        colorize::Color::BrightGreen
                    } else if duration < Duration::from_millis(100) {
                        colorize::Color::Green
                    } else if duration < Duration::from_millis(500) {
                        colorize::Color::Yellow
                    } else if duration < Duration::from_millis(1000) {
                        colorize::Color::Magenta
                    } else {
                        colorize::Color::Red
                    };

                    global_fg(color);

                    print!("{}{}", s, " ".repeat(20 - s.len()));

                    // FIXME Why is alignment bugged ?

                    global_fg(colorize::Color::Default);
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

use std::{
    fmt::Display,
    fs,
    time::{Duration, Instant},
};

/// Solution to an aoc problem.
#[derive(PartialEq, Eq, Debug)]
pub enum Solution {
    /// A numeric solution.
    ///
    /// # Example
    /// ```
    /// # use aoc::Solution;
    /// let s = Solution::Num(52);
    /// let s = Solution::from(52u32);
    /// let s = Solution::from(52i32);
    /// let s = Solution::from(52u64);
    /// let s = Solution::from(52i64);
    /// ```
    Num(i64),

    /// A textual solution.
    ///
    /// # Example
    /// ```
    /// # use aoc::Solution;
    /// let s = Solution::from("Xt454dDdz56");
    /// let s = Solution::from("
    /// #######################\n\
    /// ###    ##    ##     ###\n\
    /// ###### ## ##### #######\n\
    /// ####   ##    ##     ###\n\
    /// ###### ## ## ###### ###\n\
    /// ###    ##    ##     ###\n\
    /// #######################\n\
    /// ");
    /// ```
    String(String),

    /// Indicates when a function fails to solve the problem,
    /// e.g. fir djisktra, dfs, ...
    NotFound,

    /// For a solution that is not yet implemented.\
    /// This is particulary usefull part1 is being implemented, but part2 isn't yet.
    NotImplemented,
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Solution {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Num(x) => write!(f, "{x}"),
            Solution::String(s) => write!(f, "{s}"),
            Solution::NotFound => write!(f, "Not found"),
            Solution::NotImplemented => write!(f, "Not implemented yet"),
        }
    }
}

/// Nice workaround for avoiding conflicting implementations :
/// https://stackoverflow.com/questions/39159226/conflicting-implementations-of-trait-in-rust
pub trait Numeric {}
impl Numeric for i32 {}
impl Numeric for u32 {}
impl Numeric for i64 {}
impl Numeric for u64 {}

impl<T> From<T> for Solution
where
    T: num_traits::NumCast + Numeric,
{
    fn from(value: T) -> Self {
        Self::Num(num_traits::cast(value).unwrap())
    }
}

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

/// Wrapper to represent an aoc day solution.
pub enum AocImplementation {
    /// Day implemented in Rust, containing part 1 and part 2 functions.
    Some(SolFuncType, SolFuncType),
    /// Contains an option for the language name.
    ImplementedInOtherLanguage(Option<&'static str>),
    NotImplementedYet,
}

pub struct AocDay {
    pub name: &'static str,
    pub implementation: AocImplementation,
}

fn ex_function(
    solve_func: SolFuncType,
    input_path: &str,
) -> Result<(Solution, Duration), std::io::Error> {
    match fs::read_to_string(input_path) {
        Ok(input) => {
            let now = Instant::now();
            let result = solve_func(input);
            Ok((result, now.elapsed()))
        }
        Err(x) => Err(x),
    }
}

fn print_execution(solve_func: SolFuncType, input_path: &str, name: &str) {
    match ex_function(solve_func, input_path) {
        Ok((sol, duration)) => match sol {
            sol @ Solution::Num(_) | sol @ Solution::String(_) | sol @ Solution::NotFound => {
                println!(
                    "{name} -> {sol} {}, {:.2?}",
                    " ".repeat(20 - sol.to_string().len()),
                    duration
                );
            }
            Solution::NotImplemented => println!("{name} -> not implemented yet"),
        },
        Err(err) => eprintln!("Failed to execute {name} for input {input_path} : {}", err),
    }
}

/// Computes the project relative path for a day input.
///
/// Input files for `YYYY_DD` are expected to be stored
/// at `data/year_YYYY/dayDD/input.txt`.\
/// Test input files are expected to be stored the same way,
/// just named `test_input_ID.txt`.
///
/// # Arguments
///
/// * `name` - Expected format of a day name : YYYY_DD.,
///  e.g. `2015_01`, `2018_20`
/// * `option_id` - If this is an example input, specify its id.
fn get_path(name: &str, option_id: Option<usize>) -> String {
    assert_eq!(name.len(), 7);
    assert_eq!(name.get(4..5).unwrap(), "_");

    format!(
        "data/year_{}/day{}/{}.txt",
        name.get(0..4).unwrap(),
        name.get(5..7).unwrap(),
        match option_id {
            Some(id) => format!("test_input{}", id),
            None => "input".to_string(),
        }
    )
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

impl AocDay {
    pub fn test_input(&self, part: usize, test_id: usize, expected_result: Solution) {
        if let AocImplementation::Some(part1, part2) = self.implementation {
            let test_input_path = get_path(self.name, Some(test_id));
            match ex_function(
                match part {
                    1 => part1,
                    2 => part2,
                    x => panic!("Part {} ? There's no such part in an aoc problem !", x),
                },
                &test_input_path,
            ) {
                Ok((result, _)) => assert_eq!(
                    result, expected_result,
                    "Wrong result : expected {}, got {}",
                    expected_result, result
                ),
                Err(err) => panic!(
                    "Failed to execute {} for test input {test_input_path} : {}",
                    self.name, err
                ),
            }
        } else {
            panic!("{} part {} not implemented !", self.name, part);
        }
    }

    pub fn solve(&self) {
        println!("--- {} ---", self.name);
        match self.implementation {
            AocImplementation::Some(part1, part2) => {
                let input_path = get_path(self.name, None);
                print_execution(part1, &input_path, "part 1");
                print_execution(part2, &input_path, "part 2");
            }
            _ => (),
        }
        println!("{}", self.implementation);
    }

    pub fn solve_for_benchmark(&self, part: usize) {
        let input_path = &get_path(self.name, None);

        match self.implementation {
            AocImplementation::Some(part1, part2) => match part {
                0 => {
                    let _ = ex_function(part1, input_path);
                }
                1 => {
                    let _ = ex_function(part2, input_path);
                }
                x => panic!("Part {} ? There's no such part in an aoc problem !", x),
            },
            AocImplementation::ImplementedInOtherLanguage(_)
            | AocImplementation::NotImplementedYet => panic!("{}", self.implementation),
        }
    }
}

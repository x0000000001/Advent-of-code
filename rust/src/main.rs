use aoc::{DayExecutionResult, Solution, DAYS};
use clap::{Parser, Subcommand};
use colorize::global_fg;
use std::time::Duration;

const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = 2024;

#[derive(Parser)]
#[command(
    name = "AOC",
    about = "Execute various Advent of Code challenges",
    author = "Your Name",
    version = "1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Execute both parts of a specific day
    Day { year: usize, day: usize },
    /// Execute both parts of a day with a specific test_id
    Test {
        year: usize,
        day: usize,
        test_id: usize,
    },
    /// Execute both parts of a day on all existing test inputs
    TestAll { year: usize, day: usize },
    /// Execute every day of a given year (only execution time)
    Year { year: usize },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Day { year, day } => day_command(*year, *day),
        Command::Test { year, day, test_id } => test(*year, *day, *test_id),
        Command::TestAll { year, day } => test_all(*year, *day),
        Command::Year { year } => year_command(*year),
    }
}

fn format_day_key(year: usize, day: usize) -> String {
    format!("{}_{}", year, format!("{:02}", day))
}

fn print_line((sol, duration): (Solution, Duration), name: &str) {
    match sol {
        Solution::Num(_) | Solution::String(_) | Solution::NotFound => println!(
            "{name} -> {sol} {}, {:.2?}",
            " ".repeat(20 - sol.to_string().len()),
            duration
        ),
        Solution::NotImplemented | Solution::Day25Part2 => println!("{}", sol),
    }
}

fn day_command(year: usize, day: usize) {
    let name = format_day_key(year, day);

    if let Some(day) = DAYS.get(&name) {
        println!("--- {} ---", name);
        match day.solve() {
            DayExecutionResult::Success(result1, result2) => {
                print_line(result1, "part 1");
                print_line(result2, "part 2");
            }
            DayExecutionResult::ImplementationError(implementation) => println!("{implementation}"),
            DayExecutionResult::FileReadingError(s_err) => {
                println!("Looks like there is no input for this file : {s_err}")
            }
        }
    } else {
        println!("Can't find day {}.", name);
    }
}

fn test(year: usize, day: usize, test_id: usize) {
    let name = format_day_key(year, day);

    if let Some(day) = DAYS.get(&name) {
        println!("--- {}  | test {} ---", name, test_id);
        match day.solve_test(test_id) {
            DayExecutionResult::Success(result1, result2) => {
                print_line(result1, "part 1");
                print_line(result2, "part 2");
            }
            DayExecutionResult::ImplementationError(implementation) => println!("{implementation}"),
            DayExecutionResult::FileReadingError(s_err) => {
                println!(
                    "Looks like there is no test input with id {test_id} for day {name} : {s_err}"
                )
            }
        }
    } else {
        println!("Can't find day {}.", name);
    }
}

fn test_all(year: usize, day: usize) {
    let name = format_day_key(year, day);

    if let Some(day) = DAYS.get(&name) {
        let execution_results = day.solve_all_tests();
        if execution_results.is_empty() {
            println!("There isn't any test for day {}.", day.name);
        } else {
            for (test_id, execution_result) in execution_results.into_iter().enumerate() {
                println!("--- {}  | test {} ---", name, test_id);
                match execution_result {
                    DayExecutionResult::Success(result1, result2) => {
                        print_line(result1, "part 1");
                        print_line(result2, "part 2");
                    }
                    DayExecutionResult::ImplementationError(implementation) => {
                        println!("{implementation}")
                    }
                    DayExecutionResult::FileReadingError(s_err) => {
                        println!("Looks like there is no test input with id {test_id} for day {name} : {s_err}")
                    }
                }
                println!();
            }
        }
    } else {
        println!("Can't find day {}.", name);
    }
}

fn year_command(year: usize) {
    if year < FIRST_YEAR {
        println!("AOC starts in {}. Year {} doesn't exist.", FIRST_YEAR, year);
        return;
    } else if year > LAST_YEAR {
        println!(
            "AOC currently stops at year {}. Year {} doesn't exist yet :)",
            LAST_YEAR, year
        );
        return;
    }

    let days_of_year: Vec<&aoc::AocDay> = DAYS
        .keys()
        .filter(|k| k.starts_with(&year.to_string()))
        .filter_map(|k| DAYS.get(k))
        .collect();

    if days_of_year.is_empty() {
        println!("There are no days here for year {}. Sorry !", year);
        return;
    }

    println!(
        "{}{}{}{}",
        " ".repeat(11),
        "Part 1",
        " ".repeat(18),
        "Part 2"
    );
    println!("{}", "-".repeat(53));

    for day in days_of_year {
        print_one_line_day(day.solve(), &day.name);
    }
}

fn print_one_line_day(day_result: DayExecutionResult, day_name: &str) {
    print!("{} || ", day_name);

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

        print!(
            "{}{}",
            s,
            " ".repeat(
                (20 + if duration < Duration::from_millis(1) {
                    1
                } else {
                    0
                }) - s.len()
            )
        );

        global_fg(colorize::Color::Default);
    };

    match day_result {
        DayExecutionResult::Success(result1, result2) => {
            print_time(result1);
            print!("|| ");
            print_time(result2);
            println!();
        }
        DayExecutionResult::ImplementationError(implementation) => println!("{implementation}"),
        DayExecutionResult::FileReadingError(_) => {
            println!("no input")
        }
    }
}

// TODO command "test" to see a day's test_inputs results on input listed in data
// TODO move display from aoc_days to here
// TODO implement tests for days with test result from AOC website (very long)

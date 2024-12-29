use aoc::{helpers::Input, AocDay, DayExecutionResult, DAYS};
use clap::{Parser, Subcommand};
use colored::Colorize;
use colorize::global_fg;
use itertools::Itertools;
use std::{cmp::Ordering, time::Duration};

const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = 2024;

const USER: &'static str = "AntoineMines";

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
    /// Execute all days
    All,
}

fn main() {
    let cli = Cli::parse();

    run_jobs(
        &DAYS
            .iter()
            .filter(|d| match &cli.command {
                &Command::Day { year, day }
                | &Command::TestAll { year, day }
                | &Command::Test {
                    year,
                    day,
                    test_id: _,
                } => d.year == year && d.day == day,
                &Command::Year { year } => d.year == year,
                Command::All => true,
            })
            .flat_map(|&d| match &cli.command {
                Command::Day { year: _, day: _ } | Command::Year { year: _ } | Command::All => {
                    vec![(
                        d.clone(),
                        Input {
                            user: USER,
                            test_id: None,
                        },
                    )]
                }
                &Command::Test {
                    year: _,
                    day: _,
                    test_id,
                } => vec![(
                    d.clone(),
                    Input {
                        user: USER,
                        test_id: Some(test_id),
                    },
                )],
                Command::TestAll { year: _, day: _ } => Input::get_all(USER, d)
                    .into_iter()
                    .map(|i| (d.clone(), i))
                    .collect(),
            })
            .collect(),
    );
}

fn time_color(duration: Duration) -> &'static str {
    if duration < Duration::from_millis(1) {
        "green"
    } else if duration < Duration::from_millis(100) {
        "yellow"
    } else {
        "red"
    }
}

/// Computes and displays results for requested jobs.
///
/// # Arguments
///
/// * `jobs` - Pairs of (day, Option<test_id>). If no test id is specified, the
fn run_jobs(jobs: &Vec<(AocDay, Input)>) {
    for (day, _) in jobs.iter() {
        let year = day.year;

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
    }

    println!(
        "{:15} || {:30} | {:30} || {:30} | {:30}",
        " ", "Part 1", "", "Part 2", ""
    );
    println!(
        "{:15} || {:30} | {:30} || {:30} | {:30}",
        " ", "Time", "Result", "Time", "Result"
    );
    println!("{}", "=".repeat(149));

    let mut current_year = None;

    for (day, input) in jobs.iter().sorted_by(|(day0, input0), (day1, input1)| {
        // sort by year, then by day, then by test_id
        day0.year.cmp(&day1.year).then(day0.day.cmp(&day1.day).then(
            match (input0.test_id, input1.test_id) {
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
                (Some(a), Some(b)) => a.cmp(&b),
            },
        ))
    }) {
        if current_year.is_none_or(|y| y != day.year) {
            current_year = Some(day.year);
            println!("{:-^149}", day.year.to_string());
        }

        print_one_line_day(day, &day.solve(input));
    }
}

fn print_one_line_day(day: &AocDay, day_result: &DayExecutionResult) {
    let name = format!("{} {}", day.year, day.day);

    match day_result {
        DayExecutionResult::Success((sol0, duration0), (sol1, duration1)) => {
            global_fg(colorize::Color::Red);
            println!(
                "{:15} || {:30} | {:30} || {:30} | {:30}",
                name,
                format!("{:.0?}", duration0).color(time_color(*duration0)),
                sol0,
                format!("{:.0?}", duration1).color(time_color(*duration1)),
                sol1
            );
        }
        DayExecutionResult::ImplementationError(implementation) => println!(
            "{:15} || {:30} | {:30} || {:30} | {:30}",
            name, implementation, "-", "-", "-"
        ),
        DayExecutionResult::FileReadingError(_) => {
            println!(
                "{:15} || {:30} | {:30} || {:30} | {:30}",
                name, "no input", "-", "-", "-"
            );
        }
    }
}

// TODO command "test" to see a day's test_inputs results on input listed in data
// TODO move display from aoc_days to here
// TODO implement tests for days with test result from AOC website (very long)

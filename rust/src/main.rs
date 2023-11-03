use std::{env, time::Duration};

use aoc::{DayExecutionResult, Solution, DAYS};
use colorize::global_fg;

const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = 2022;

const HELP: &'static str = "
AOC

Usage: 
aoc [target]
where target can be one of the following :
- \"day YYYY_DD\" for executing both parts of year YYYY day DD
- \"test YYYY_DD test_id\" for executing both parts of year YYYY day DD with test_input_<test_id>.txt
- \"test_all YYYY_DD\" for executing both parts of year YYYY day DD on all existing test inputs
- \"year YYYY\" for executing every day of a given year (only execution time)

Examples:
aoc all
aoc day 2015_01
aoc year 2017
";

type CommandFunc = fn(Vec<String>);

use phf::phf_map;

const COMMANDS: phf::Map<&'static str, CommandFunc> = phf_map! {
    "day" => day,
    "test" => test,
    "test_all" => test_all,
    "year" => year,
};

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

fn day(args: Vec<String>) {
    let print_line = |(sol, duration): (Solution, Duration), name: &str| match sol {
        Solution::Num(_) | Solution::String(_) | Solution::NotFound => println!(
            "{name} -> {sol} {}, {:.2?}",
            " ".repeat(20 - sol.to_string().len()),
            duration
        ),
        Solution::NotImplemented | Solution::Day25Part2 => println!("{}", sol),
    };

    let name = &args[2];

    let day = match DAYS.get(name) {
        Some(day) => day,
        None => {
            println!("Can't find day {}.", name);
            return;
        }
    };

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
}

fn test(args: Vec<String>) {
    let name = &args[2];

    let day = match DAYS.get(name) {
        Some(day) => day,
        None => {
            println!("Can't find day {}.", name);
            return;
        }
    };

    let test_id = match args[3].parse::<usize>() {
        Ok(i) => i,
        Err(_) => {
            println!("Please provide a test id as a natural number.");
            return;
        }
    };

    println!("--- {}  | test {} ---", name, test_id);

    match day.solve_test(test_id) {
        DayExecutionResult::Success(result1, result2) => {
            print_line(result1, "part 1");
            print_line(result2, "part 2");
        }
        DayExecutionResult::ImplementationError(implementation) => println!("{implementation}"),
        DayExecutionResult::FileReadingError(s_err) => {
            println!("Looks like there is no test input with id {test_id} for day {name} : {s_err}")
        }
    }
}

fn test_all(args: Vec<String>) {
    let name = &args[2];

    let day = match DAYS.get(name) {
        Some(day) => day,
        None => {
            println!("Can't find day {}.", name);
            return;
        }
    };

    let execution_results = day.solve_all_tests();

    if execution_results.len() == 0 {
        println!("There isn't any test for day {}.", day.name);
    }

    for (test_id, execution_result) in execution_results.into_iter().enumerate() {
        println!("--- {}  | test {} ---", name, test_id);

        match execution_result {
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

        println!()
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

        // This is a dirty fix for microseconds
        // being printed on less characters than
        // they appear to to String.
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

fn year(args: Vec<String>) {
    let year = match args[2].parse::<usize>() {
        Ok(y) => y,
        Err(_) => {
            println!("Wrong format for year : {}. Expected YYYY.", args[2]);
            return;
        }
    };

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

    let mut days_of_year: Vec<&aoc::AocDay> = DAYS
        .keys()
        .filter(|k| k[0..4] == args[2])
        .map(|k| DAYS.get(k).unwrap())
        .collect();

    days_of_year.sort_by_key(|d| d.name);

    if days_of_year.len() == 0 {
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

    days_of_year
        .iter()
        .for_each(|d| print_one_line_day(d.solve(), d.name));
}

fn main() {
    println!();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{HELP}");
        return;
    };

    if let Some(func) = COMMANDS.get(&args[1]) {
        func(args);
    } else {
        println!("{HELP}");
        return;
    }

    println!();

    // TODO command "test" to see a day's test_inputs results on input listed in data
    // TODO move display from aoc_days to here
    // TODO implement tests for days with test result from AOC website (very long)
}

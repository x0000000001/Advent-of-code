use std::env;

use aoc::DAYS;

const FIRST_YEAR: usize = 2015;
const LAST_YEAR: usize = 2022;

const HELP: &'static str = "
AOC

Usage: 
    aoc [target]
    where target can be one of the following :
     - \"all\" for executing all days
     - \"day YYYY_DD\" for executing both parts of year YYYY day DD
     - \"year YYYY\" for executing every day of a given year 

Examples:
    aoc all
    aoc day 2015_01
    aoc year 2017
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{HELP}");
        return;
    };

    match args[1].as_str() {
        "all" => {
            for day in DAYS.values() {
                day.solve(aoc::OutputType::Full);
            }
        }

        "day" => {
            let name = &args[2];
            if let Some(day) = DAYS.get(name) {
                day.solve(aoc::OutputType::Full);
            } else {
                println!("Can't find day {}.", name);
                return;
            }
        }

        "year" => {
            if let Ok(year) = args[2].parse::<usize>() {
                if year < FIRST_YEAR {
                    println!("AOC starts in {} ! {} doesn't exist.", FIRST_YEAR, year);
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
                    .filter(|k| k[0..4] == args[2])
                    .map(|k| DAYS.get(k).unwrap())
                    .collect();

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

                for day in days_of_year {
                    day.solve(aoc::OutputType::OneLineTime);
                }
            } else {
                println!("Wrong format for year : {}. Expected YYYY.", args[2]);
                return;
            }
        }
        _ => println!("{HELP}"),
    }
}

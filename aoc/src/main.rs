use std::env;

use aoc::DAYS;

const HELP: &'static str = "
AOC

Usage: 
    aoc [target]
    where target can be one of the following :
     - \"all\" for executing all days
     - \"YYYY_DD\" for executing both parts of year YYYY day DD

Examples:
    aoc all
    aoc 2015_01
    aoc 2019_20
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{HELP}");
        return;
    };

    if args[1] == "all" {
        for day in DAYS.values() {
            day.solve();
        }

        return;
    }

    for name in args[1..args.len()].iter() {
        if let Some(day) = DAYS.get(&name) {
            day.solve();
        } else {
            println!("Can't find day {}.", name);
        }
    }
}

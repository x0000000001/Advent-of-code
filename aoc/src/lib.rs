pub mod helpers;

pub mod year_2015;

pub use helpers::{AocDay, AocImplementation, OutputType, Solution};

use phf::phf_map;

pub const DAYS: phf::Map<&'static str, AocDay> = phf_map! {
    // 2015
    "2015_01" =>AocDay {name: "2015_01", implementation: AocImplementation::Some(year_2015::day01::part1, year_2015::day01::part2)},
    "2015_02" =>AocDay {name: "2015_02", implementation: AocImplementation::Some(year_2015::day02::part1, year_2015::day02::part2)},
    "2015_03" =>AocDay {name: "2015_03", implementation: AocImplementation::Some(year_2015::day03::part1, year_2015::day03::part2)},
    "2015_04" =>AocDay {name: "2015_04", implementation: AocImplementation::Some(year_2015::day04::part1, year_2015::day04::part2)},
    "2015_05" =>AocDay {name: "2015_05", implementation: AocImplementation::Some(year_2015::day05::part1, year_2015::day05::part2)},
    "2015_06" =>AocDay {name: "2015_06", implementation: AocImplementation::Some(year_2015::day06::part1, year_2015::day06::part2)},
    "2015_07" =>AocDay {name: "2015_07", implementation: AocImplementation::Some(year_2015::day07::part1, year_2015::day07::part2)},
    "2015_08" =>AocDay {name: "2015_08", implementation: AocImplementation::Some(year_2015::day08::part1, year_2015::day08::part2)},
    "2015_09" =>AocDay {name: "2015_09", implementation: AocImplementation::Some(year_2015::day09::part1, year_2015::day09::part2)},
    "2015_10" =>AocDay {name: "2015_10", implementation: AocImplementation::Some(year_2015::day10::part1, year_2015::day10::part2)},
    "2015_11" =>AocDay {name: "2015_11", implementation: AocImplementation::Some(year_2015::day11::part1, year_2015::day11::part2)},
    "2015_12" =>AocDay {name: "2015_12", implementation: AocImplementation::Some(year_2015::day12::part1, year_2015::day12::part2)},
    "2015_13" =>AocDay {name: "2015_13", implementation: AocImplementation::Some(year_2015::day13::part1, year_2015::day13::part2)},
    "2015_14" =>AocDay {name: "2015_14", implementation: AocImplementation::Some(year_2015::day14::part1, year_2015::day14::part2)},
    "2015_15" =>AocDay {name: "2015_15", implementation: AocImplementation::Some(year_2015::day15::part1, year_2015::day15::part2)},
    "2015_16" =>AocDay {name: "2015_16", implementation: AocImplementation::Some(year_2015::day16::part1, year_2015::day16::part2)},
    "2015_17" =>AocDay {name: "2015_17", implementation: AocImplementation::Some(year_2015::day17::part1, year_2015::day17::part2)},
    "2015_18" =>AocDay {name: "2015_18", implementation: AocImplementation::Some(year_2015::day18::part1, year_2015::day18::part2)},
    "2015_19" =>AocDay {name: "2015_19", implementation: AocImplementation::Some(year_2015::day19::part1, year_2015::day19::part2)},
    "2015_20" =>AocDay {name: "2015_20", implementation: AocImplementation::Some(year_2015::day20::part1, year_2015::day20::part2)},
    "2015_21" =>AocDay {name: "2015_21", implementation: AocImplementation::Some(year_2015::day21::part1, year_2015::day21::part2)},
    "2015_22" =>AocDay {name: "2015_22", implementation: AocImplementation::Some(year_2015::day22::part1, year_2015::day22::part2)},
    "2015_23" =>AocDay {name: "2015_23", implementation: AocImplementation::Some(year_2015::day23::part1, year_2015::day23::part2)},
    "2015_24" =>AocDay {name: "2015_24", implementation: AocImplementation::Some(year_2015::day24::part1, year_2015::day24::part2)},
    "2015_25" =>AocDay {name: "2015_25", implementation: AocImplementation::Some(year_2015::day25::part1, year_2015::day25::part2)},
};

pub mod helpers;

// TODO add verbal mod to display prints from days when wanted ?

pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2018;
pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;
pub mod year_2023;

pub use helpers::{AocDay, AocImplementation, DayExecutionResult, OutputType, Solution};

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
    // 2016
    "2016_01" =>AocDay {name: "2016_01", implementation: AocImplementation::Some(year_2016::day01::part1, year_2016::day01::part2)},
    "2016_02" =>AocDay {name: "2016_02", implementation: AocImplementation::Some(year_2016::day02::part1, year_2016::day02::part2)},
    "2016_03" =>AocDay {name: "2016_03", implementation: AocImplementation::Some(year_2016::day03::part1, year_2016::day03::part2)},
    "2016_04" =>AocDay {name: "2016_04", implementation: AocImplementation::Some(year_2016::day04::part1, year_2016::day04::part2)},
    "2016_05" =>AocDay {name: "2016_05", implementation: AocImplementation::Some(year_2016::day05::part1, year_2016::day05::part2)},
    "2016_06" =>AocDay {name: "2016_06", implementation: AocImplementation::Some(year_2016::day06::part1, year_2016::day06::part2)},
    "2016_07" =>AocDay {name: "2016_07", implementation: AocImplementation::Some(year_2016::day07::part1, year_2016::day07::part2)},
    "2016_08" =>AocDay {name: "2016_08", implementation: AocImplementation::Some(year_2016::day08::part1, year_2016::day08::part2)},
    "2016_09" =>AocDay {name: "2016_09", implementation: AocImplementation::Some(year_2016::day09::part1, year_2016::day09::part2)},
    "2016_10" =>AocDay {name: "2016_10", implementation: AocImplementation::Some(year_2016::day10::part1, year_2016::day10::part2)},
    "2016_11" =>AocDay {name: "2016_11", implementation: AocImplementation::Some(year_2016::day11::part1, year_2016::day11::part2)},
    "2016_12" =>AocDay {name: "2016_12", implementation: AocImplementation::Some(year_2016::day12::part1, year_2016::day12::part2)},
    "2016_13" =>AocDay {name: "2016_13", implementation: AocImplementation::Some(year_2016::day13::part1, year_2016::day13::part2)},
    "2016_14" =>AocDay {name: "2016_14", implementation: AocImplementation::Some(year_2016::day14::part1, year_2016::day14::part2)},
    "2016_15" =>AocDay {name: "2016_15", implementation: AocImplementation::Some(year_2016::day15::part1, year_2016::day15::part2)},
    "2016_16" =>AocDay {name: "2016_16", implementation: AocImplementation::Some(year_2016::day16::part1, year_2016::day16::part2)},
    "2016_17" =>AocDay {name: "2016_17", implementation: AocImplementation::Some(year_2016::day17::part1, year_2016::day17::part2)},
    "2016_18" =>AocDay {name: "2016_18", implementation: AocImplementation::Some(year_2016::day18::part1, year_2016::day18::part2)},
    "2016_19" =>AocDay {name: "2016_19", implementation: AocImplementation::Some(year_2016::day19::part1, year_2016::day19::part2)},
    "2016_20" =>AocDay {name: "2016_20", implementation: AocImplementation::Some(year_2016::day20::part1, year_2016::day20::part2)},
    "2016_21" =>AocDay {name: "2016_21", implementation: AocImplementation::Some(year_2016::day21::part1, year_2016::day21::part2)},
    "2016_22" =>AocDay {name: "2016_22", implementation: AocImplementation::Some(year_2016::day22::part1, year_2016::day22::part2)},
    "2016_23" =>AocDay {name: "2016_23", implementation: AocImplementation::Some(year_2016::day23::part1, year_2016::day23::part2)},
    "2016_24" =>AocDay {name: "2016_24", implementation: AocImplementation::Some(year_2016::day24::part1, year_2016::day24::part2)},
    "2016_25" =>AocDay {name: "2016_25", implementation: AocImplementation::Some(year_2016::day25::part1, year_2016::day25::part2)},
    // 2017
    "2017_01" =>AocDay {name: "2017_01", implementation: AocImplementation::Some(year_2017::day01::part1, year_2017::day01::part2)},
    "2017_02" =>AocDay {name: "2017_02", implementation: AocImplementation::Some(year_2017::day02::part1, year_2017::day02::part2)},
    "2017_03" =>AocDay {name: "2017_03", implementation: AocImplementation::Some(year_2017::day03::part1, year_2017::day03::part2)},
    "2017_04" =>AocDay {name: "2017_04", implementation: AocImplementation::Some(year_2017::day04::part1, year_2017::day04::part2)},
    "2017_05" =>AocDay {name: "2017_05", implementation: AocImplementation::Some(year_2017::day05::part1, year_2017::day05::part2)},
    "2017_06" =>AocDay {name: "2017_06", implementation: AocImplementation::Some(year_2017::day06::part1, year_2017::day06::part2)},
    "2017_07" =>AocDay {name: "2017_07", implementation: AocImplementation::Some(year_2017::day07::part1, year_2017::day07::part2)},
    "2017_08" =>AocDay {name: "2017_08", implementation: AocImplementation::Some(year_2017::day08::part1, year_2017::day08::part2)},
    "2017_09" =>AocDay {name: "2017_09", implementation: AocImplementation::Some(year_2017::day09::part1, year_2017::day09::part2)},
    "2017_10" =>AocDay {name: "2017_10", implementation: AocImplementation::Some(year_2017::day10::part1, year_2017::day10::part2)},
    "2017_11" =>AocDay {name: "2017_11", implementation: AocImplementation::Some(year_2017::day11::part1, year_2017::day11::part2)},
    "2017_12" =>AocDay {name: "2017_12", implementation: AocImplementation::Some(year_2017::day12::part1, year_2017::day12::part2)},
    "2017_13" =>AocDay {name: "2017_13", implementation: AocImplementation::Some(year_2017::day13::part1, year_2017::day13::part2)},
    "2017_14" =>AocDay {name: "2017_14", implementation: AocImplementation::Some(year_2017::day14::part1, year_2017::day14::part2)},
    "2017_15" =>AocDay {name: "2017_15", implementation: AocImplementation::Some(year_2017::day15::part1, year_2017::day15::part2)},
    "2017_16" =>AocDay {name: "2017_16", implementation: AocImplementation::Some(year_2017::day16::part1, year_2017::day16::part2)},
    "2017_17" =>AocDay {name: "2017_17", implementation: AocImplementation::Some(year_2017::day17::part1, year_2017::day17::part2)},
    "2017_18" =>AocDay {name: "2017_18", implementation: AocImplementation::Some(year_2017::day18::part1, year_2017::day18::part2)},
    "2017_19" =>AocDay {name: "2017_19", implementation: AocImplementation::Some(year_2017::day19::part1, year_2017::day19::part2)},
    "2017_20" =>AocDay {name: "2017_20", implementation: AocImplementation::Some(year_2017::day20::part1, year_2017::day20::part2)},
    "2017_21" =>AocDay {name: "2017_21", implementation: AocImplementation::Some(year_2017::day21::part1, year_2017::day21::part2)},
    "2017_22" =>AocDay {name: "2017_22", implementation: AocImplementation::Some(year_2017::day22::part1, year_2017::day22::part2)},
    "2017_23" =>AocDay {name: "2017_23", implementation: AocImplementation::Some(year_2017::day23::part1, year_2017::day23::part2)},
    "2017_24" =>AocDay {name: "2017_24", implementation: AocImplementation::Some(year_2017::day24::part1, year_2017::day24::part2)},
    "2017_25" =>AocDay {name: "2017_25", implementation: AocImplementation::Some(year_2017::day25::part1, year_2017::day25::part2)},
    // 2018
    "2018_01" =>AocDay {name: "2018_01", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_02" =>AocDay {name: "2018_02", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_03" =>AocDay {name: "2018_03", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_04" =>AocDay {name: "2018_04", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_05" =>AocDay {name: "2018_05", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_06" =>AocDay {name: "2018_06", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_07" =>AocDay {name: "2018_07", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_08" =>AocDay {name: "2018_08", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_09" =>AocDay {name: "2018_09", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_10" =>AocDay {name: "2018_10", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_11" =>AocDay {name: "2018_11", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_12" =>AocDay {name: "2018_12", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2018_13" =>AocDay {name: "2018_13", implementation: AocImplementation::Some(year_2018::day13::part1, year_2018::day13::part2)},
    "2018_14" =>AocDay {name: "2018_14", implementation: AocImplementation::Some(year_2018::day14::part1, year_2018::day14::part2)},
    "2018_15" =>AocDay {name: "2018_15", implementation: AocImplementation::Some(year_2018::day15::part1, year_2018::day15::part2)},
    "2018_16" =>AocDay {name: "2018_16", implementation: AocImplementation::Some(year_2018::day16::part1, year_2018::day16::part2)},
    "2018_17" =>AocDay {name: "2018_17", implementation: AocImplementation::Some(year_2018::day17::part1, year_2018::day17::part2)},
    "2018_18" =>AocDay {name: "2018_18", implementation: AocImplementation::Some(year_2018::day18::part1, year_2018::day18::part2)},
    "2018_19" =>AocDay {name: "2018_19", implementation: AocImplementation::Some(year_2018::day19::part1, year_2018::day19::part2)},
    "2018_20" =>AocDay {name: "2018_20", implementation: AocImplementation::Some(year_2018::day20::part1, year_2018::day20::part2)},
    "2018_21" =>AocDay {name: "2018_21", implementation: AocImplementation::Some(year_2018::day21::part1, year_2018::day21::part2)},
    "2018_22" =>AocDay {name: "2018_22", implementation: AocImplementation::Some(year_2018::day22::part1, year_2018::day22::part2)},
    "2018_23" =>AocDay {name: "2018_23", implementation: AocImplementation::Some(year_2018::day23::part1, year_2018::day23::part2)},
    "2018_24" =>AocDay {name: "2018_24", implementation: AocImplementation::Some(year_2018::day24::part1, year_2018::day24::part2)},
    "2018_25" =>AocDay {name: "2018_25", implementation: AocImplementation::Some(year_2018::day25::part1, year_2018::day25::part2)},
    // 2019
    "2019_01" =>AocDay {name: "2019_01", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_02" =>AocDay {name: "2019_02", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_03" =>AocDay {name: "2019_03", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_04" =>AocDay {name: "2019_04", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_05" =>AocDay {name: "2019_05", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_06" =>AocDay {name: "2019_06", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_07" =>AocDay {name: "2019_07", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2019_08" =>AocDay {name: "2019_08", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2019_09" =>AocDay {name: "2019_09", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2019_10" =>AocDay {name: "2019_10", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2019_11" =>AocDay {name: "2019_11", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2019_12" =>AocDay {name: "2019_12", implementation: AocImplementation::Some(year_2019::day12::part1, year_2019::day12::part2)},
    "2019_13" =>AocDay {name: "2019_13", implementation: AocImplementation::Some(year_2019::day13::part1, year_2019::day13::part2)},
    "2019_14" =>AocDay {name: "2019_14", implementation: AocImplementation::Some(year_2019::day14::part1, year_2019::day14::part2)},
    "2019_15" =>AocDay {name: "2019_15", implementation: AocImplementation::Some(year_2019::day15::part1, year_2019::day15::part2)},
    "2019_16" =>AocDay {name: "2019_16", implementation: AocImplementation::Some(year_2019::day16::part1, year_2019::day16::part2)},
    "2019_17" =>AocDay {name: "2019_17", implementation: AocImplementation::Some(year_2019::day17::part1, year_2019::day17::part2)},
    "2019_18" =>AocDay {name: "2019_18", implementation: AocImplementation::Some(year_2019::day18::part1, year_2019::day18::part2)},
    "2019_19" =>AocDay {name: "2019_19", implementation: AocImplementation::Some(year_2019::day19::part1, year_2019::day19::part2)},
    "2019_20" =>AocDay {name: "2019_20", implementation: AocImplementation::Some(year_2019::day20::part1, year_2019::day20::part2)},
    "2019_21" =>AocDay {name: "2019_21", implementation: AocImplementation::Some(year_2019::day21::part1, year_2019::day21::part2)},
    "2019_22" =>AocDay {name: "2019_22", implementation: AocImplementation::Some(year_2019::day22::part1, year_2019::day22::part2)},
    "2019_23" =>AocDay {name: "2019_23", implementation: AocImplementation::Some(year_2019::day23::part1, year_2019::day23::part2)},
    "2019_24" =>AocDay {name: "2019_24", implementation: AocImplementation::Some(year_2019::day24::part1, year_2019::day24::part2)},
    "2019_25" =>AocDay {name: "2019_25", implementation: AocImplementation::Some(year_2019::day25::part1, year_2019::day25::part2)},
    // 2020
    "2020_01" =>AocDay {name: "2020_01", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_02" =>AocDay {name: "2020_02", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_03" =>AocDay {name: "2020_03", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_04" =>AocDay {name: "2020_04", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_05" =>AocDay {name: "2020_05", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_06" =>AocDay {name: "2020_06", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_07" =>AocDay {name: "2020_07", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_08" =>AocDay {name: "2020_08", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_09" =>AocDay {name: "2020_09", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_10" =>AocDay {name: "2020_10", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_11" =>AocDay {name: "2020_11", implementation: AocImplementation::ImplementedInOtherLanguage(Some("JavaScript"))},
    "2020_12" =>AocDay {name: "2020_12", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_13" =>AocDay {name: "2020_13", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_14" =>AocDay {name: "2020_14", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_15" =>AocDay {name: "2020_15", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_16" =>AocDay {name: "2020_16", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_17" =>AocDay {name: "2020_17", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_18" =>AocDay {name: "2020_18", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_19" =>AocDay {name: "2020_19", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2020_20" =>AocDay {name: "2020_20", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2020_21" =>AocDay {name: "2020_21", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2020_22" =>AocDay {name: "2020_22", implementation: AocImplementation::Some(year_2020::day22::part1, year_2020::day22::part2)},
    "2020_23" =>AocDay {name: "2020_23", implementation: AocImplementation::Some(year_2020::day23::part1, year_2020::day23::part2)},
    "2020_24" =>AocDay {name: "2020_24", implementation: AocImplementation::Some(year_2020::day24::part1, year_2020::day24::part2)},
    "2020_25" =>AocDay {name: "2020_25", implementation: AocImplementation::Some(year_2020::day25::part1, year_2020::day25::part2)},
     // 2021
    "2021_01" =>AocDay {name: "2021_01", implementation: AocImplementation::Some(year_2021::day01::part1, year_2021::day01::part2)},
    "2021_02" =>AocDay {name: "2021_02", implementation: AocImplementation::Some(year_2021::day02::part1, year_2021::day02::part2)},
    "2021_03" =>AocDay {name: "2021_03", implementation: AocImplementation::Some(year_2021::day03::part1, year_2021::day03::part2)},
    "2021_04" =>AocDay {name: "2021_04", implementation: AocImplementation::Some(year_2021::day04::part1, year_2021::day04::part2)},
    "2021_05" =>AocDay {name: "2021_05", implementation: AocImplementation::Some(year_2021::day05::part1, year_2021::day05::part2)},
    "2021_06" =>AocDay {name: "2021_06", implementation: AocImplementation::Some(year_2021::day06::part1, year_2021::day06::part2)},
    "2021_07" =>AocDay {name: "2021_07", implementation: AocImplementation::Some(year_2021::day07::part1, year_2021::day07::part2)},
    "2021_08" =>AocDay {name: "2021_08", implementation: AocImplementation::Some(year_2021::day08::part1, year_2021::day08::part2)},
    "2021_09" =>AocDay {name: "2021_09", implementation: AocImplementation::Some(year_2021::day09::part1, year_2021::day09::part2)},
    "2021_10" =>AocDay {name: "2021_10", implementation: AocImplementation::Some(year_2021::day10::part1, year_2021::day10::part2)},
    "2021_11" =>AocDay {name: "2021_11", implementation: AocImplementation::Some(year_2021::day11::part1, year_2021::day11::part2)},
    "2021_12" =>AocDay {name: "2021_12", implementation: AocImplementation::Some(year_2021::day12::part1, year_2021::day12::part2)},
    "2021_13" =>AocDay {name: "2021_13", implementation: AocImplementation::Some(year_2021::day13::part1, year_2021::day13::part2)},
    "2021_14" =>AocDay {name: "2021_14", implementation: AocImplementation::Some(year_2021::day14::part1, year_2021::day14::part2)},
    "2021_15" =>AocDay {name: "2021_15", implementation: AocImplementation::Some(year_2021::day15::part1, year_2021::day15::part2)},
    "2021_16" =>AocDay {name: "2021_16", implementation: AocImplementation::Some(year_2021::day16::part1, year_2021::day16::part2)},
    "2021_17" =>AocDay {name: "2021_17", implementation: AocImplementation::Some(year_2021::day17::part1, year_2021::day17::part2)},
    "2021_18" =>AocDay {name: "2021_18", implementation: AocImplementation::Some(year_2021::day18::part1, year_2021::day18::part2)},
    "2021_19" =>AocDay {name: "2021_19", implementation: AocImplementation::Some(year_2021::day19::part1, year_2021::day19::part2)},
    "2021_20" =>AocDay {name: "2021_20", implementation: AocImplementation::Some(year_2021::day20::part1, year_2021::day20::part2)},
    "2021_21" =>AocDay {name: "2021_21", implementation: AocImplementation::Some(year_2021::day21::part1, year_2021::day21::part2)},
    "2021_22" =>AocDay {name: "2021_22", implementation: AocImplementation::Some(year_2021::day22::part1, year_2021::day22::part2)},
    "2021_23" =>AocDay {name: "2021_23", implementation: AocImplementation::Some(year_2021::day23::part1, year_2021::day23::part2)},
    "2021_24" =>AocDay {name: "2021_24", implementation: AocImplementation::Some(year_2021::day24::part1, year_2021::day24::part2)},
    "2021_25" =>AocDay {name: "2021_25", implementation: AocImplementation::Some(year_2021::day25::part1, year_2021::day25::part2)},
     // 2022
    "2022_01" =>AocDay {name: "2022_01", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_02" =>AocDay {name: "2022_02", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_03" =>AocDay {name: "2022_03", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_04" =>AocDay {name: "2022_04", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_05" =>AocDay {name: "2022_05", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_06" =>AocDay {name: "2022_06", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_07" =>AocDay {name: "2022_07", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_08" =>AocDay {name: "2022_08", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2022_09" =>AocDay {name: "2022_09", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_10" =>AocDay {name: "2022_10", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_11" =>AocDay {name: "2022_11", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_12" =>AocDay {name: "2022_12", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_13" =>AocDay {name: "2022_13", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_14" =>AocDay {name: "2022_14", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_15" =>AocDay {name: "2022_15", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_16" =>AocDay {name: "2022_16", implementation: AocImplementation::ImplementedInOtherLanguage(Some("TypeScript"))},
    "2022_17" =>AocDay {name: "2022_17", implementation: AocImplementation::Some(year_2022::day17::part1, year_2022::day17::part2)},
    "2022_18" =>AocDay {name: "2022_18", implementation: AocImplementation::Some(year_2022::day18::part1, year_2022::day18::part2)},
    "2022_19" =>AocDay {name: "2022_19", implementation: AocImplementation::Some(year_2022::day19::part1, year_2022::day19::part2)},
    "2022_20" =>AocDay {name: "2022_20", implementation: AocImplementation::Some(year_2022::day20::part1, year_2022::day20::part2)},
    "2022_21" =>AocDay {name: "2022_21", implementation: AocImplementation::Some(year_2022::day21::part1, year_2022::day21::part2)},
    "2022_22" =>AocDay {name: "2022_22", implementation: AocImplementation::Some(year_2022::day22::part1, year_2022::day22::part2)},
    "2022_23" =>AocDay {name: "2022_23", implementation: AocImplementation::Some(year_2022::day23::part1, year_2022::day23::part2)},
    "2022_24" =>AocDay {name: "2022_24", implementation: AocImplementation::Some(year_2022::day24::part1, year_2022::day24::part2)},
    "2022_25" =>AocDay {name: "2022_25", implementation: AocImplementation::Some(year_2022::day25::part1, year_2022::day25::part2)},
      // 2023
    "2023_01" =>AocDay {name: "2023_01", implementation: AocImplementation::NotImplementedYet},
    "2023_02" =>AocDay {name: "2023_02", implementation: AocImplementation::NotImplementedYet},
    "2023_03" =>AocDay {name: "2023_03", implementation: AocImplementation::NotImplementedYet},
    "2023_04" =>AocDay {name: "2023_04", implementation: AocImplementation::NotImplementedYet},
    "2023_05" =>AocDay {name: "2023_05", implementation: AocImplementation::NotImplementedYet},
    "2023_06" =>AocDay {name: "2023_06", implementation: AocImplementation::NotImplementedYet},
    "2023_07" =>AocDay {name: "2023_07", implementation: AocImplementation::NotImplementedYet},
    "2023_08" =>AocDay {name: "2023_08", implementation: AocImplementation::NotImplementedYet},
    "2023_09" =>AocDay {name: "2023_09", implementation: AocImplementation::NotImplementedYet},
    "2023_10" =>AocDay {name: "2023_10", implementation: AocImplementation::NotImplementedYet},
    "2023_11" =>AocDay {name: "2023_11", implementation: AocImplementation::NotImplementedYet},
    "2023_12" =>AocDay {name: "2023_12", implementation: AocImplementation::NotImplementedYet},
    "2023_13" =>AocDay {name: "2023_13", implementation: AocImplementation::NotImplementedYet},
    "2023_14" =>AocDay {name: "2023_14", implementation: AocImplementation::NotImplementedYet},
    "2023_15" =>AocDay {name: "2023_15", implementation: AocImplementation::NotImplementedYet},
    "2023_16" =>AocDay {name: "2023_16", implementation: AocImplementation::NotImplementedYet},
    "2023_17" =>AocDay {name: "2023_17", implementation: AocImplementation::NotImplementedYet},
    "2023_18" =>AocDay {name: "2023_18", implementation: AocImplementation::NotImplementedYet},
    "2023_19" =>AocDay {name: "2023_19", implementation: AocImplementation::NotImplementedYet},
    "2023_20" =>AocDay {name: "2023_20", implementation: AocImplementation::NotImplementedYet},
    "2023_21" =>AocDay {name: "2023_21", implementation: AocImplementation::NotImplementedYet},
    "2023_22" =>AocDay {name: "2023_22", implementation: AocImplementation::NotImplementedYet},
    "2023_23" =>AocDay {name: "2023_23", implementation: AocImplementation::NotImplementedYet},
    "2023_24" =>AocDay {name: "2023_24", implementation: AocImplementation::NotImplementedYet},
    "2023_25" =>AocDay {name: "2023_25", implementation: AocImplementation::NotImplementedYet},
};

// TODO lines! macro for let lines: Vec<String> = s
// .lines()
// .into_iter()
// .map(|line| line.trim().to_owned())
// .collect();

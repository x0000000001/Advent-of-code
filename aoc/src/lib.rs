pub mod helpers;

pub mod year_2015;
pub mod year_2016;

pub use helpers::{AocDay, AocImplementation, OutputType, Solution};

use phf::phf_map;

pub const DAYS: phf::Map<&'static str, AocDay> = phf_map! {
    "2015_01" =>AocDay {name: "2015_01", implementation: AocImplementation::Some(year_2015::day01::part1, year_2015::day01::part2)},
    // "2015_02" =>AocDay {name: "2015_02", implementation: AocImplementation::Some(year_2015::day02::part1, year_2015::day02::part2)},
    // "2016_01" =>AocDay {name: "2016_01", implementation: AocImplementation::Some(year_2016::day01::part1, year_2016::day01::part2)},
    // "2016_02" =>AocDay {name: "2016_02", implementation: AocImplementation::Some(year_2016::day02::part1, year_2016::day02::part2)},
    "2015_02" =>AocDay {name: "2015_02", implementation: AocImplementation::ImplementedInOtherLanguage(Some("Go"))},
    "2016_01" =>AocDay {name: "2016_01", implementation: AocImplementation::NotImplementedYet},
    "2016_02" =>AocDay {name: "2016_02", implementation: AocImplementation::ImplementedInOtherLanguage(None)},
};

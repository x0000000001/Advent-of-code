use std::{collections::HashMap, fs};

type Stone = usize;
type Blinks = usize;
type InputType = Vec<Stone>;

fn count(stone: Stone, blinks: Blinks, seen: &mut HashMap<(Stone, Blinks), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(&count) = seen.get(&(stone, blinks)) {
        return count;
    }

    let result;

    if stone == 0 {
        result = count(1, blinks - 1, seen);
    } else {
        let digits_count = stone.checked_ilog10().unwrap_or(0) + 1;

        if digits_count % 2 == 0 {
            let p = (10 as usize).pow(digits_count / 2);
            let right = stone % p;
            result = count(right, blinks - 1, seen) + count((stone - right) / p, blinks - 1, seen);
        } else {
            result = count(stone * 2024, blinks - 1, seen);
        }
    }

    seen.insert((stone, blinks), result);

    result
}

pub fn solve(v: Vec<usize>) {
    let mut seen = HashMap::new();
    println!(
        "{} {}",
        v.iter()
            .map(|&stone| count(stone, 25, &mut seen))
            .sum::<usize>(),
        v.iter()
            .map(|&stone| count(stone, 75, &mut seen))
            .sum::<usize>()
    )
}

fn parse(s: String) -> InputType {
    s.split_whitespace().map(|w| w.parse().unwrap()).collect()
}

fn main() {
    for file in ["data/input_example.txt", "data/input.txt"] {
        // for file in ["data/input_example.txt"] {
        let s = fs::read_to_string(file).unwrap();
        solve(parse(s.clone()));
    }
}

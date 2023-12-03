use dayrust::{part1, part2};

fn main() {
    println!("TESTS");
    let s = String::from(include_str!("../test_input.txt"));
    println!("{}", part1(s.clone()));
    println!("{}", part2(s.clone()));
    println!("REAL INPUT");
    let s = String::from(include_str!("../input.txt"));
    println!("{}", part1(s.clone()));
    println!("{}", part2(s.clone()));
}

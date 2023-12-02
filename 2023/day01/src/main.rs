use dayrust::{part1, part2};

fn main() {
    println!("PART1");
    let s = String::from(include_str!("../test_input.txt"));
    println!("test input : {}", part1(s.clone()));
    let s = String::from(include_str!("../input.txt"));
    println!("real input : {}", part1(s.clone()));
    println!("PART2");
    let s = String::from(include_str!("../test_input1.txt"));
    println!("test input : {}", part2(s.clone()));
    let s = String::from(include_str!("../input.txt"));
    println!("real input : {}", part2(s.clone()));
}

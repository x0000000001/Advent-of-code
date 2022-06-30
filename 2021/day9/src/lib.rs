use std::fs;

const FILE: &str = "input.txt";

pub fn result_1() -> i64
{  
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let height_map:Vec<Vec<u8>> = input.iter().
        map(|line| line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect())
        .collect();
    
    

    0
}


pub fn result_2() -> i64
{   
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();
    
    0
}
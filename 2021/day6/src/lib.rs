use std::fs;

const FILE: &str = "input.txt";

pub fn result_1() -> i32
{  
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();
    
    0
}


pub fn result_2() -> i32
{   
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    0
}
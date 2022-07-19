use day25::*;
use std::time::Instant;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn ex_function(foo: fn(InputType) -> i64, name: &str){
    let now = Instant::now();
    let result = foo(read_input(INPUT_PATH)) as i64;
    println!("{name} -> {result} {}, {:.2?}", " ".repeat(20 - result.to_string().len()), now.elapsed());
}


fn main() {
    ex_function(result_1, "result 1");
    ex_function(result_2, "result 2");
}


pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let mut input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input[0] = input[0].replace(",", "");
    input[0] = input[0].replace(".", "");
    let w = input[0].split_whitespace().collect::<Vec<&str>>();

    (w[w.len()-3].parse().unwrap(), w.last().unwrap().parse().unwrap())
}
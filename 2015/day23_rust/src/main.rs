use day23::*;
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

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut res: InputType = vec![];

    for l in input {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        let instr = match w[0] {
            "hlf" => 0,
            "tpl" => 1,
            "inc" => 2,
            "jmp" => 3,
            "jie" => 4,
            "jio" => 5,
            _  => -1
        };

        let arg0 = if instr == 3 { w[1].parse().unwrap()} else {
            match w[1] {
                "a," => 0,
                "b," => 1,
                "a" => 0,
                "b" => 1,
                _ => -1
            }
        };

        let arg1 = if instr == 4 || instr == 5 {
            w[2].parse().unwrap()
        } else {-1};

        res.push((instr,arg0,arg1));
    }

    res
}
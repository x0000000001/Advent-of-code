use std::fs;

const FILE: &str = "input.txt";

fn read_input() -> Vec<String>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input
}

pub fn result_1() -> i64
{
    let input = read_input();
    let mut c = 0;

    for s in input[0].chars() {
        if s == '(' {
            c += 1;
        }else{
            c -= 1
        }
    }
    
    c
}


pub fn result_2() -> i64
{   
    let input = read_input();
    let mut c = 0;
    let mut i = 0;

    for s in input[0].chars() {
        i+=1;
        if s == '(' {
            c += 1;
        }else{
            c -= 1
        }
        if c == -1 {
            return i;
        }
    }
    
    0
}
use day1::*;
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
    input[0].replace(",", "").split_whitespace().map(
        |s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap())
    ).collect()
}


#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test1()
    {
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 12);
        assert_eq!(result_1(Vec::from([('R',2),('R',2),('R',2)])), 2);
        assert_eq!(result_1(Vec::from([('R',2),('L',3)])), 5);
        assert_eq!(result_1(Vec::from([('R',456),('R',456)])), 912);
        assert_eq!(result_1(Vec::from([('R',456),('R',0),('R',456)])), 0);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(Vec::from([('R',8),('R',4),('R',4),('R',8)])), 4);
    }
}
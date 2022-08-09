use day08::*;
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
        let w: Vec<&str> = l.split_whitespace().collect();
        if w[0] == "rect" {
            let w1: Vec<&str> = w[1].split("x").collect();
            res.push((0,w1[0].parse().unwrap(),w1[1].parse().unwrap()));
        } else {
            let begin: usize = w[2].split("=").last().unwrap().parse().unwrap();
            let amplitude: usize = w.last().unwrap().parse().unwrap();

            if w[1] == "row" {
                res.push((1,begin,amplitude))
            } else {
                res.push((2,begin,amplitude))
            }
        }
    }

    res
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
        assert_eq!(result_1(read_input(INPUT_PATH)), 0);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}
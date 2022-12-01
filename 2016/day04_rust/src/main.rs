use day04::*;
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

    for l in input{
        let w = l.split("-").collect::<Vec<&str>>();
        let w1 = w.last().unwrap().split("[").collect::<Vec<&str>>();

        res.push((
            w[0..(w.len()-1)].into_iter().map(|el| el.to_string()).collect(),
            w1[1][0..(w1[1].len()-1)].to_string().chars().collect(),
            w1[0].parse::<i64>().unwrap(),
    ));
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 0);
    }
    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}
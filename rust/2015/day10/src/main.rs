use day10::*;
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

    input[0].chars().map(|c| c.to_digit(10).unwrap()).collect()
}


#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test_step()
    {
        let mut input = Vec::from([1]);
        step(&mut input);
        assert_eq!(input, Vec::from([1,1]));

        step(&mut input);
        assert_eq!(input, Vec::from([2,1]));

        step(&mut input);
        assert_eq!(input, Vec::from([1,2,1,1]));

        step(&mut input);
        assert_eq!(input, Vec::from([1,1,1,2,2,1]));

        step(&mut input);
        assert_eq!(input, Vec::from([3,1,2,2,1,1]));
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}
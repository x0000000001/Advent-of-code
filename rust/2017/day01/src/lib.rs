use std::fs;

pub type InputType = Vec<u8>;

pub fn result_1(input: InputType) -> i64
{
    let mut sum: i64 = 0;

    for i in 0..input.len() {
        if input[i] == input[(i+1)%input.len()] {
            sum += input[i] as i64;
        }
    }
    
    sum
}


pub fn result_2(input: InputType) -> i64
{   
    let mut sum: i64 = 0;
    let len = input.len();

    for i in 0..len {
        if input[i] == input[(i+len/2)%len] {
            sum += input[i] as i64;
        }
    }
    
    sum
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input[0].chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
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
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 0);
    }
}
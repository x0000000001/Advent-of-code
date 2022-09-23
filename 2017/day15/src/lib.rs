use std::fs;

pub type InputType = (u64,u64);

pub fn result_1(input: InputType) -> i64
{
    let mut a = input.0;
    let mut b = input.1;
    let mut score = 0;

    for _ in 0..40000000 {
        a = (a * 16807)%2147483647;
        b = (b * 48271)%2147483647;

        if a%65536 == b%65536 {
            score += 1;
        }
    }
    
    score
}


pub fn result_2(input: InputType) -> i64
{   
    let mut a = input.0;
    let mut b = input.1;
    let mut score = 0;

    for _ in 0..5000000 {

        loop {
            a = (a * 16807)%2147483647;
            
            if a%4 == 0 {
                break;
            }
        }

        
        loop {
            b = (b * 48271)%2147483647;

            if b%8==0 {
                break;
            }
        }

        if a%65536 == b%65536 {
            score += 1;
        }
    }
    
    score
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    fn get_val(s: &str) -> u64 {
        let w = s.split_whitespace();
        w.last().unwrap().parse().unwrap()
    }

    (get_val(&input[0]),get_val(&input[1]))
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
        assert_eq!(result_1((65,8921)), 588);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2((65,8921)), 309);
    }
}
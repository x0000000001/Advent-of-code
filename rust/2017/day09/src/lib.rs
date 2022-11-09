use std::fs;

pub type InputType = String;

pub fn result_1(input: InputType) -> i64
{
    let mut total_score = 0;
    let mut current_score = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip_next = true,
                _ => ()
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '{' => current_score += 1,
                '}' => {
                    total_score += current_score;
                    current_score -= 1;
                },
                _ => ()
            }
        }
    }
    
    total_score
}


pub fn result_2(input: InputType) -> i64
{   
    let mut in_garbage = false;
    let mut skip_next = false;
    let mut count = 0;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip_next = true,
                _ => count += 1
            }
        } else {
            match c {
                '<' => in_garbage = true,
                _ => ()
            }
        }
    }
    
    count
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let mut input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();
    
    input.pop().unwrap()
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 9);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 0);
    }
}
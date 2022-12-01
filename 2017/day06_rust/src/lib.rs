use std::fs;

pub type InputType = Vec<u64>;

fn transition(input: &mut InputType) {
    // reversing to respect the "first wins" rule
    let max_index = input.len() - 1 - input.iter().rev().enumerate().max_by_key(|(_,x)| *x).unwrap().0;
    let mut stock = input[max_index];
    input[max_index] = 0;
    let mut give_index = (max_index + 1)%input.len();

    while stock > 0 {
        input[give_index] += 1;
        give_index  = (give_index + 1)%input.len();
        stock -= 1;
    }
}

pub fn result_1(mut input: InputType) -> i64
{
    let mut seen: Vec<InputType> = vec![];
    let mut count = 0;

    while !seen.contains(&input) {
        seen.push(input.clone());
        transition(&mut input);
        count += 1;
    }

    count
}


pub fn result_2(mut input: InputType) -> i64
{   
    let mut seen: Vec<InputType> = vec![];
    let mut count = 0;

    while !seen.contains(&input) {
        seen.push(input.clone());
        transition(&mut input);
        count += 1;
    }

    count - seen.into_iter().position(|x| x == input).unwrap() as i64
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input[0].split_whitespace().map(|el| el.parse().unwrap()).collect()
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 5);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 4);
    }
}
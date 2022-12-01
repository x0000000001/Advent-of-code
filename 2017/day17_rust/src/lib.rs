use std::fs;

pub type InputType = usize;

pub fn result_1(input: InputType) -> i64
{
    let mut v = vec![];
    v.push(0);
    let mut index = 0;

    for i in 1..2018 {
        index = (index + input)%v.len();
        index += 1;
        v.insert(index, i);
    }

    return v[(index+1)%2018];
}


pub fn result_2(input: InputType) -> i64
{   
    let mut value_next_to_0 = 0;
    let mut index = 0;

    for i in 1..50000000 {
        index = (index + input)%i;

        if index == 0 {
            value_next_to_0 = i;
        }

        index += 1;
    }
    
    value_next_to_0 as i64
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input[0].parse().unwrap()
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
        assert_eq!(result_1(3), 638);
    }
}
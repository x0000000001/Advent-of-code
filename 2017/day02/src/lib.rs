use std::fs;

pub type InputType = Vec<Vec<i64>>;

pub fn result_1(input: InputType) -> i64
{
    let mut sum = 0;

    for l in input {
        sum += l.iter().max().unwrap() - l.iter().min().unwrap();
    }

    sum
}


pub fn result_2(input: InputType) -> i64
{   
    let mut sum = 0;

    'line_loop: for l in input {
        for i in 0..l.len() {
            for j in 0..l.len() {
                if i == j {
                    continue;
                }
                if l[i]%l[j] == 0 {
                    sum += l[i]/l[j];
                    continue 'line_loop;
                }
            }
        }
    }

    sum
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut res: InputType = vec![vec![];input.len()];

    for i in 0..input.len() {
        res[i] = input[i].split_whitespace().map(|str| str.parse().unwrap()).collect();
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
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 0);
    }
}
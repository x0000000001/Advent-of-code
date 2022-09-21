use std::fs;

pub type InputType = Vec<(u64,u64)>;

pub fn result_1(input: InputType) -> i64
{
    let mut severity = 0;
    
    for (depth,range) in input {
        if depth%(2*(range-1)) == 0 {
            severity += depth * range;
        }
    }
    
    severity as i64
}

/// That's pretty much force-brute... is there a better way ? (missing around with modulos)
pub fn result_2(input: InputType) -> i64
{   
    let mut i = 0;

    'main_loop: loop {
        for (depth,range) in input.iter() {
            if (depth+i)%(2*(range-1)) == 0 {
                i += 1;
                continue 'main_loop;
            }
        }

        break;
    }

    i as i64
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input.into_iter().map(|mut l| {
        l = l.replace(":", "");
        let mut w = l.split_whitespace();
        (w.next().unwrap().parse().unwrap(),w.next().unwrap().parse().unwrap())
    }).collect()
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 24);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 10);
    }
}
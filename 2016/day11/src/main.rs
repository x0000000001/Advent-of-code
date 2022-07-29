use day11::*;
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
        let mut floor: Vec<(bool,String)> = vec![];
        let l = l.replace(",", "");
        let l = l.replace(".", "");
        let l = l.replace("and", "");
        let w: Vec<&str> = l.split_whitespace().collect();

        let mut i = 5;

        if w[i] == "relevant" {
            continue;
        }

        while i < w.len() {
            let is_microchip = w[i+1] == "microchip";
            if is_microchip {
                floor.push((is_microchip, 
                    w[i].split("-").
                    collect::<Vec<&str>>()[0].to_string()));
            } else {
                floor.push((is_microchip, w[i].to_string()));
            }
            i+=3
        }

        res.push(floor);
    }

    while res.len() != 4 {
        res.push(vec![]);
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 11);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}
use day16::*;
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

    let mut aunts: InputType = vec![];

    for l in input {
        let l = l.replace(",", "");
        let l = l.replace(":", "");
        let mut words = l.split_whitespace();
        words.next();
        words.next();

        let mut next_aunt = AuntSue::default();

        while let Some(selector) = words.next() {
            let count = words.next().unwrap().parse::<i64>().unwrap();

            match selector {
                "children" => next_aunt.children = count,
                "cats" => next_aunt.cats = count,
                "samoyeds" => next_aunt.samoyeds = count,
                "pomeranians" => next_aunt.pomeranians = count,
                "akitas" => next_aunt.akitas = count,
                "vizslas" => next_aunt.vizslas = count,
                "goldfish" => next_aunt.goldfish = count,
                "trees" => next_aunt.trees = count,
                "cars" => next_aunt.cars = count,
                "perfumes" => next_aunt.perfumes = count,
                _ => ()
            }
        }

        aunts.push(next_aunt);
    }

    aunts
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
        assert_eq!(result_1(read_input(INPUT_PATH)), 0);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), 0);
    }
}
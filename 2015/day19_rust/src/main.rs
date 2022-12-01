use day19::*;
use std::collections::HashMap;
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

    let mut combinations: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..input.len() - 2 {
        let words = input[i].split_whitespace().collect::<Vec<&str>>();

        let mol = words[0].to_string();
        let follow = words[2].to_string();

        let accessor = combinations.entry(mol).or_insert(vec![]);
        accessor.push(follow);
    }

    (input.last().unwrap().clone(), combinations)
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 4);
        let mut new_input = read_input(TEST_INPUT_PATH);
        new_input.0 = "HOHOHO".to_string();
        assert_eq!(result_1(new_input), 7);
    }

    
    #[test]
    fn test2()
    {
        let mut new_input = ("HOH".to_string(), HashMap::from([
            ("e".to_string(), Vec::from(["O".to_string(),"H".to_string()])),
            ("H".to_string(), Vec::from(["OH".to_string(),"HO".to_string()])),
            ("O".to_string(), Vec::from(["HH".to_string()]))
        ]));

        assert_eq!(result_2(new_input.clone()), 3);
        new_input.0 = "HOHOHO".to_string();
        assert_eq!(result_2(new_input), 6);
    }
}
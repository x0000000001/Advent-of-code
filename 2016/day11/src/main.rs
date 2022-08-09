use day11::*;
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

    let mut res: InputType = (vec![],0);
    let mut names_to_u64: HashMap<String,u64> = HashMap::new();
    let mut id = 0;
     
    for l in input {
        let mut floor: Vec<(bool,u64)> = vec![];
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
            let name = if is_microchip { 
                    w[i].split("-").
                    collect::<Vec<&str>>()[0].to_string()
            } else {
                w[i].to_string()
            };

            let id = if let Some(existing_id) = names_to_u64.get(&name) {
                *existing_id
            } else {
                names_to_u64.insert(name, id);
                id += 1;
                id - 1
            };

            floor.push((is_microchip,id));

            i+=3
        }

        res.0.push(floor);
    }

    while res.0.len() != 4 {
        res.0.push(vec![]);
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
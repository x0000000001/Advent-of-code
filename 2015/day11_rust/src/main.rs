use day11::*;
use std::time::Instant;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn ex_function(foo: fn(InputType) -> String, name: &str){
    let now = Instant::now();
    let result = foo(read_input(INPUT_PATH));
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

    let mut input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input.remove(0)
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), "abcdffaa");
    }

    #[test]
    fn isvalid()
    {
        assert_eq!(is_valid(&"hijklmmn".to_string()), false);
        assert_eq!(is_valid(&"abbceffg".to_string()), false);
        assert_eq!(is_valid(&"abbcegjk".to_string()), false);
        assert_eq!(is_valid(&"ghjaabcc".to_string()), true);
    }

    #[test]
    fn testnext()
    {
        let mut s = "xyz".to_string();
        next(&mut s, 2);
        assert_eq!(s, "xza");

        next(&mut s, 2);
        assert_eq!(s, "xzb");

        s = "xzz".to_string();
        next(&mut s, 2);
        assert_eq!(s, "yaa");


        s = "ghjaabcb".to_string();
        next(&mut s, 7);
        assert_eq!(s, "ghjaabcc");

    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(INPUT_PATH)), "");
    }
}
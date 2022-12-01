use day8::*;
use std::time::Instant;
use std::fs;





pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input
}





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

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test_real_chars()
    {
        assert_eq!(count_real_chars("aaa"), 3);
        assert_eq!(count_real_chars("aaa\\\"aaa"), 7);
        assert_eq!(count_real_chars("\\x27"), 1);
        assert_eq!(count_real_chars("\\x27aaa\\\"aaa"), 8);
    }

    #[test]
    fn test1()
    {
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 12);
    }

    #[test]
    fn test_added_chars()
    {
        assert_eq!(count_added_chars("aaa"), 5);
        assert_eq!(count_added_chars(""), 2);
        assert_eq!(count_added_chars("aaa\\\"aaa"), 12);
        assert_eq!(count_added_chars("\\x27"), 7);
    }
    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 19);
    }
}
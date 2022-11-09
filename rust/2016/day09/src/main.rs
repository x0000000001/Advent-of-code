use day09::*;
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

    input.into_iter().flat_map(|el| el.chars().collect::<Vec<char>>()).collect::<Vec<char>>()
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
        assert_eq!(result_1("ADVENT".chars().collect()), 6);
        assert_eq!(result_1("A(1x5)BC".chars().collect()), 7);
        assert_eq!(result_1("(3x3)XYZ".chars().collect()), 9);
        assert_eq!(result_1("A(2x2)BCD(2x2)EFG".chars().collect()), 11);
        assert_eq!(result_1("(6x1)(1x3)A".chars().collect()), 6);
        assert_eq!(result_1("X(8x2)(3x3)ABCY".chars().collect()), 18);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2("(3x3)XYZ".chars().collect()), 9);
        assert_eq!(result_2("X(8x2)(3x3)ABCY".chars().collect()), 20);
        assert_eq!(result_2("(27x12)(20x12)(13x14)(7x10)(1x12)A".chars().collect()), 241920);
        assert_eq!(result_2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".chars().collect()), 445);
    }
}
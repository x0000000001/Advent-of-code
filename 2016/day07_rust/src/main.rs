use day07::*;
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

    input
}


#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test_suport_tls()
    {
        assert_eq!(supports_tls("abba[mnop]qrst"), true);
        assert_eq!(supports_tls("abcd[bddb]xyyx"), false);
        assert_eq!(supports_tls("aaaa[qwer]tyui"), false);
        assert_eq!(supports_tls("ioxxoj[asdfgh]zxcvbn"), true);
    }

    
    #[test]
    fn test_suport_ssl()
    {
        assert_eq!(supports_ssl("aba[bab]xyz"), true);
        assert_eq!(supports_ssl("xyx[xyx]xyx"), false);
        assert_eq!(supports_ssl("aaa[kek]eke"), true);
        assert_eq!(supports_ssl("zazbz[bzb]cdb"), true);
    }
}
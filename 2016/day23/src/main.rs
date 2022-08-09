use day23::*;
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
        let w: Vec<&str> = l.split_whitespace().collect();

        let get_instr_val = |inp: &str| -> InstrVal {
            if let Ok(val) = inp.parse() {
                InstrVal::Val(val)
            } else {
                InstrVal::Index(
                match inp {
                    "a" => 0, "b" => 1, "c" => 2, "d" => 3, _ => panic!("Wrong index.")
                }
            )
            }
        };

        let x =  get_instr_val(w[1]);

        let mut y: InstrVal = InstrVal::None;
        
        let instr = match w[0] {
            "cpy" => 0,
            "inc" => 1,
            "dec" => 2,
            "jnz" => 3,
            "tgl" => 4,
            _ => panic!("Wrong instruction.")
        };

        if [0,3].contains(&instr) {
            y = get_instr_val(w[2]);
        }
        
        res.push((instr,x,y));
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
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 3);
    }
}
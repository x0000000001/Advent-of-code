use std::{fs, collections::HashMap};

pub type InputType = (Vec<char>,Vec<DanseMove>);

pub enum DanseMove {
    Spin(usize),
    Exchange(usize,usize),
    Partner(char,char)
}

fn apply_move(v: &mut Vec<char>, m: &DanseMove) {
    match m {
        DanseMove::Spin(x) => {
            v.rotate_right(*x);
        }, 
        DanseMove::Exchange(x, y) => {
            v.swap(*x, *y);
        },
        DanseMove::Partner(c0, c1) => {
            let index0 = v.iter().position(|&r| r == *c0).unwrap();
            let index1 = v.iter().position(|&r| r == *c1).unwrap();

            v.swap(index0, index1);
        }
    }
}

fn apply_moves(v: &mut Vec<char>, moves: &Vec<DanseMove>, count: usize) {

    let mut i = 0;
    // <node, step it has been encountered>
    let mut seen: HashMap<Vec<char>, usize> = HashMap::new();

    while i < count {
        if let Some(last_encounter) = seen.get(v) {
            let step = i-last_encounter;

            while (i+step) <= count {
                i += step;
            }
        } else {
            seen.insert(v.clone(), i);
        }

        for m in moves.iter() {
            apply_move(v, m);
        }

        i += 1;
    }
}

pub fn result_1((mut v, moves): InputType) -> i64
{
    apply_moves(&mut v, &moves, 1);
    let result: String = v.into_iter().collect();
    println!("{}", result);

    0
}


pub fn result_2((mut v, moves): InputType) -> i64
{   
    apply_moves(&mut v, &moves, 1000000000);
    let result: String = v.into_iter().collect();
    println!("{}", result);

    0
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let res: Vec<DanseMove> = input[0].split(",").map(|w|

        {
            let command = w.chars().next().unwrap();
            let words = w[1..].split("/").collect::<Vec<&str>>();

            match command {
                's' => DanseMove::Spin(words[0].parse().unwrap()),
                'x' => DanseMove::Exchange(words[0].parse().unwrap(), words[1].parse().unwrap()),
                'p' => DanseMove::Partner(words[0].chars().next().unwrap(), words[1].chars().next().unwrap()),
                _ => panic!()
            }
        }

    ).collect();

    ("abcdefghijklmnop".chars().collect(),res)
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
        assert_eq!(result_1(("abcde".chars().collect(),Vec::from([DanseMove::Spin(1),DanseMove::Exchange(3, 4),DanseMove::Partner('e', 'b')]))), 0);
    }

    
    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 0);
    }
}
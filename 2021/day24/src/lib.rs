use std::{fs};

const FILE: &str = "input.txt";


fn read_input() -> Vec<(i64,i64,i64)> 
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    // vec of (m,n,p)
    let mut ns: Vec<(i64, i64, i64)> = vec![];

    for i in 0..input.len()/18 {
        let m: i64 = input[i * 18 + 4].split_whitespace().last().unwrap().parse().unwrap();
        let n: i64 = input[i * 18 + 5].split_whitespace().last().unwrap().parse().unwrap();
        let p: i64 = input[i * 18 + 15].split_whitespace().last().unwrap().parse().unwrap();

        ns.push((m,n,p));
    }
    
    ns
}

#[allow(dead_code)]
fn step(m: i64, n: i64, p: i64, z: i64, w: i64) -> i64
{
    let mut result = z;
    let x = z % 26 + n;
    result /= m;

    if x != w {
        result = result * 26 + w + p;
    }
    result
}

fn possible_inputs(m: i64, n: i64, p: i64, goal: i64, w: i64) -> Vec<i64>
{
    let mut v: Vec<i64> = vec![];

    let mut x = goal;
    x = x - w - p;

    if x%26 == 0 {
        v.push(x / 26 * m);
    }

    if 0 <= w - n && w - n < 26 {
        v.push(w - n + goal * m);
    }

    v
}

fn digit_list_to_num(digits: Vec<u8>) -> i64 
{
    let mut i = 0;
    for d in digits {
        i = i * 10 + d as i64;
    }

    i
}

#[allow(dead_code)]
fn test_sol(input: &Vec<(i64,i64,i64)>, i: &i64) -> i64
{   
    let digits: Vec<u32> = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut z = 0;

    for((m,n,p),d) in input.iter().zip(digits.iter()) {
        z = step(*m, *n, *p, z, *d as i64);
    }

    z
}

fn get_possible_inputs(input: &mut Vec<(i64,i64,i64)>) -> Vec<i64>
{
    input.reverse();

    let mut possible_solutions: Vec<(Vec<u8>, i64)> = Vec::from([(vec![], 0)]);

    for (m,n,p) in input.iter() {
        
        let mut new_sols: Vec<(Vec<u8>, i64)> = vec![];

        for (l, goal) in possible_solutions.iter() {
            for digit in 1..10 {
                let answers = possible_inputs(*m, *n, *p, *goal, digit);
                if !answers.is_empty() {
                    for a in answers {
                        let mut new_path = l.clone();
                        new_path.push(digit as u8);
                        new_sols.push((new_path, a));
                    }
                }
            }
        }

        // println!();

        possible_solutions = new_sols;
    }

    let mut temp: Vec<Vec<u8>> = possible_solutions.into_iter().filter(|(_,g)| *g == 0).map(|(l,_)| l).collect();
    for l in &mut temp {
        l.reverse();
    }

    let ps = temp.into_iter().map(|l| digit_list_to_num(l)).collect::<Vec<i64>>();

    ps
}

pub fn result_1() -> i64
{
    let mut input = read_input();
    get_possible_inputs(&mut input).into_iter().max().unwrap()
}


pub fn result_2() -> i64
{   
    let mut input = read_input();
    get_possible_inputs(&mut input).into_iter().min().unwrap()
}

















// other attempts, not used in the final solution

#[allow(dead_code)]
fn read_input0() -> Vec<Instruction>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut instructions: Vec<Instruction> = vec![];

    for l in input{
        let op = match &l[0..3] {
            "inp" => Operation::Inp,
            "mul" => Operation::Mul,
            "add" => Operation::Add,
            "div" => Operation::Div,
            "mod" => Operation::Mo,
            "eql" => Operation::Eql,
            _ => Operation::Inp
        };
        
        let x = l.chars().nth(4).unwrap();
        let mut y: Option<String> = None;

        if op != Operation::Inp {
            y = Some(String::from(&l[6..]));
        }

        instructions.push(Instruction{
            op,
            x,
            y
        });
    }

    instructions
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Inp,
    Mul,
    Add,
    Div,
    // not mod bc it is a rust keyword ;)
    Mo,
    Eql
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    x: char,
    y: Option<String>
}

#[allow(dead_code)]
fn output(input: u64, instructions: &Vec<Instruction>) -> Vec<i64>
{
    let mut wxyz = vec![0,0,0,0];
    let mut inp: Vec<u32> = input.to_string().chars().map(|el| el.to_digit(10).unwrap()).collect();
    inp.reverse();

    for instr in instructions {
        let result_index = match instr.x {
            'w' => 0,
            'x' => 1,
            'y' => 2,
            'z' => 3,
            _=> 0
        };
        
        if instr.op == Operation::Inp {
            wxyz[result_index] = inp.pop().unwrap() as i64;
            continue;
        }

        let possible_num = instr.y.as_ref().unwrap();
        let y: i64 = possible_num.parse().unwrap_or(match possible_num.chars().next().unwrap() {
            'w' => wxyz[0],
            'x' => wxyz[1],
            'y' => wxyz[2],
            'z' => wxyz[3],
            _ => 0
        });

        match instr.op {
            Operation::Mul => {
                wxyz[result_index] = wxyz[result_index] * y;
            },
            Operation::Eql => {
                wxyz[result_index] = if wxyz[result_index] == y {1}else{0};
            },
            Operation::Div => {
                wxyz[result_index] = wxyz[result_index] / y;
            },
            Operation::Mo => {
                wxyz[result_index] = wxyz[result_index] % y;
            },
            Operation::Add => {
                wxyz[result_index] = wxyz[result_index] + y;
            },
            _ => ()
        }
    }

    // println!("{:?}", wxyz);

    wxyz
}

// returns expression of z depending of input variables
#[allow(dead_code)]
fn str_from_instructions(instructions: &Vec<Instruction>) -> Vec<String>
{
    let mut wxyz: Vec<String> = vec![String::from("0");4];

    let mut input_index = 0;

    for instr in instructions {
        let result_index = match instr.x {
            'w' => 0,
            'x' => 1,
            'y' => 2,
            'z' => 3,
            _=> 0
        };
        
        if instr.op == Operation::Inp {
            wxyz[result_index] = String::from("v") + &input_index.to_string();
            input_index += 1;
            continue;
        }

        let possible_num = instr.y.as_ref().unwrap();
        let y: String = match possible_num.chars().next().unwrap() {
            'w' => wxyz[0].clone(),
            'x' => wxyz[1].clone(),
            'y' => wxyz[2].clone(),
            'z' => wxyz[3].clone(),
            _ => possible_num.clone()
        };

        let x = &mut wxyz[result_index];
        simplify(x,&y,&instr.op);
    }

    wxyz
}

#[allow(dead_code)]
fn simplify(x: &mut String, y: &String, op: &Operation) 
{       
    match op {

    Operation::Mul => {
        if let Ok(v0) = x.parse::<i64>() {
            if let Ok(v1) = y.parse::<i64>() {
                *x = (v0 * v1).to_string();
                return;
            }
        }

        if x == "0" || y == "0" {
            *x = String::from("0");
        }else {
            *x = format!("({} * {})",x,y);
        }
    },
    Operation::Eql => {
        if let Ok(v0) = x.parse::<i64>() {
            if let Ok(v1) = y.parse::<i64>() {
                if v0 == v1 {
                    *x = String::from("1");
                }else{
                    *x = String::from("0");
                }
                return;
            }
        }
        *x = format!("({} == {})",x,y);
    },
    Operation::Div => {
        if let Ok(v0) = x.parse::<i64>() {
            if let Ok(v1) = y.parse::<i64>() {
                *x = (v0 / v1).to_string();
                return;
            }
        }
        *x = format!("({} / {})",x,y);
    },
    Operation::Mo => {
        if let Ok(v0) = x.parse::<i64>() {
            if let Ok(v1) = y.parse::<i64>() {
                *x = (v0 % v1).to_string();
                return;
            }
        }

        if x == "0" || y == "0" {
            *x = 0.to_string();
            return;
        }
        if y == "1" {
            return;
        }

        *x = format!("({} % {})",x,y);
    },
    Operation::Add => {
        if let Ok(v0) = x.parse::<i64>() {
            if let Ok(v1) = y.parse::<i64>() {
                *x = (v0 + v1).to_string();
                return;
            }
        }

        if y == "0" {
            return;
        }
        if x == "0" {
            *x = y.clone();
            return;
        }
        *x = format!("({} + {})",x,y);
    },
    _ => ()
}

}
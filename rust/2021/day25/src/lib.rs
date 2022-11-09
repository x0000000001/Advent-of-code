use std::{fs, vec};

const FILE: &str = "input.txt";

// -1 : nothing, 0 : cucumber facing east, 1 : cucumber facing north

fn read_input() -> Vec<Vec<i8>>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let (w,h) = (input.len(), input[0].len());

    let mut map: Vec<Vec<i8>> = vec![vec![-1;h];w];

    for i in 0..w {
        let chars = input[i].chars().collect::<Vec<char>>();
        for j in 0..h {
            map[i][j] = match chars[j] {
                '.' => -1,
                '>' => 0,
                _ => 1
            }
        }
    }

    map
}

#[allow(dead_code)]
fn print_cucumbers(c: &Vec<Vec<i8>>)
{
    let (w,h) = (c.len(), c[0].len());

    for i in 0..w {
        for j in 0..h {
            match c[i][j] {
                -1 => print!("."),
                0 => print!(">"),
                1 => print!("v"),
                _ => ()
            }
        }
        println!();
    }
    
    println!();
}

// returns whether cucumbers have moved or not
// 2nd vector to check which cucumbers can move
// (created outside of function for optimization)
fn step(c: &mut Vec<Vec<i8>>, can_move: &mut Vec<Vec<bool>>) -> bool
{
    let mut have_moved = false;

    let (w,h) = (c.len(), c[0].len());

    // moving east
    for i in 0..w {
        for j in 0..h {
            match c[i][j] {
                0 => {
                    let next = (j+1)%h;
                    if c[i][next] == -1 {
                        have_moved = true;
                        can_move[i][j] = true;
                    }
                }
                _ => ()
            }
        } 
    }

    // actually moving cucumbers
    for i in 0..w {
        for j in 0..h {
            if can_move[i][j] {
                let next = (j+1)%h;
                c[i][next] = 0;
                c[i][j] = -1;
                can_move[i][j] = false;
            }
        }
    }

    // moving south
    for i in 0..w {
        for j in 0..h {
            match c[i][j] {
                1 => {
                    let next = (i+1)%w;
                    if c[next][j] == -1 {
                        have_moved = true;
                        can_move[i][j] = true;
                    }
                }
                _ => ()
            }
        } 
    }

    // actually moving cucumbers
    for i in 0..w {
        for j in 0..h {
            if can_move[i][j] {
                let next = (i+1)%w;
                c[next][j] = 1;
                c[i][j] = -1;
                can_move[i][j] = false;
            }
        }
    }

    have_moved
}

pub fn result_1() -> i64
{
    let mut c = read_input();
    let (w,h) = (c.len(), c[0].len());
    let mut moved: Vec<Vec<bool>> = vec![vec![false; h]; w];

    let mut i = 1;
    while step(&mut c, &mut moved) {
        i += 1;
    }
    i
}


pub fn result_2() -> i64
{   
    let _input = read_input();
    0
}
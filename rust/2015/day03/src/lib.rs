use std::fs;

const FILE: &str = "input.txt";

fn read_input() -> Vec<char>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    return input[0].chars().collect()
}

pub fn result_1() -> i64
{
    let input = read_input();
    let mut pos: Vec<(i64,i64)> = vec![];
    pos.push((0,0));
    let mut current_pos = (0,0);
    let mut count = 1;

    for c in input {
        match c {
            '>' => current_pos.0 += 1,
            '<' => current_pos.0 -= 1,
            'v' => current_pos.1 += 1,
            _ => current_pos.1 -= 1
        }

        if !pos.contains(&current_pos) {
            count += 1;
            pos.push(current_pos);
        }
    }

    count
}


pub fn result_2() -> i64
{   
    let input = read_input();
    let mut pos: Vec<(i64,i64)> = vec![];
    pos.push((0,0));
    let mut santa_pos0 = (0,0);
    let mut santa_pos1 = (0,0);
    let mut count = 1;

    let mut switch = true;

    for c in input {
        let to_move: &mut (i64,i64) = if switch { &mut santa_pos0} else {&mut santa_pos1};
        switch = !switch;
        match c {
            '>' => to_move.0 += 1,
            '<' => to_move.0 -= 1,
            'v' => to_move.1 += 1,
            _ => to_move.1 -= 1
        }

        if !pos.contains(&to_move) {
            count += 1;
            pos.push(*to_move);
        }
    }

    count
}
use std::fs;

const FILE: &str = "input.txt";

fn read_input() -> Vec<(i64, i64, i64)> {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut dims: Vec<(i64, i64, i64)> = vec![];

    for l in input {
        let temp: Vec<i64> = l.split('x').map(|el| el.parse().unwrap()).collect();
        dims.push((temp[0], temp[1], temp[2]));
    }

    dims
}

pub fn result_1() -> i64 {
    let input = read_input();
    let mut s = 0;

    for (l, w, h) in input {
        s += 2 * l * w + 2 * w * h + 2 * h * l + (l * w * h / [l, w, h].iter().max().unwrap());
    }
    s
}

pub fn result_2() -> i64 {
    let input = read_input();
    let mut s = 0;

    for (l, w, h) in input {
        let mut temp = [l, w, h];
        temp.sort();
        s += l * w * h + 2 * temp[0] + 2 * temp[1];
    }
    s
}

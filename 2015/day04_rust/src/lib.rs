use std::fs;

const FILE: &str = "input.txt";

fn read_input() -> String {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0].clone()
}

pub fn result_1() -> i64 {
    let input = read_input();
    let mut i = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, i.to_string()));
        let h = format!("{:x}", digest);
        if h[0..5] == *"00000" {
            return i;
        }

        i += 1;
    }
}

pub fn result_2() -> i64 {
    let input = read_input();
    let mut i = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, i.to_string()));
        let h = format!("{:x}", digest);
        if h[0..6] == *"000000" {
            return i;
        }

        i += 1;
    }
}

use std::fs;

pub type InputType = Vec<SNAFU>;

pub type SNAFU = String;

fn snafu_to_i64(sn: &SNAFU) -> i64 {
    let mut n = 0;

    for c in sn.chars() {
        n *= 5;
        n += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        }
    }

    n
}

fn i64_to_snafu(mut n: i64) -> SNAFU {
    let mut current_power = 1;
    while current_power * 5 <= n.abs() {
        current_power *= 5;
    }

    let mut s = vec![];

    while current_power > 0 {
        let mut i = -4;

        for k in -2..3 {
            if (k * current_power - n).abs() < current_power / 2 + 1 {
                i = k;
                break;
            }
        }

        n -= i * current_power;
        s.push(i);
        current_power /= 5;
    }

    s.into_iter()
        .map(|i| match i {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!(),
        })
        .collect()
}

pub fn result_1(input: InputType) -> i64 {
    println!(
        "{}",
        i64_to_snafu(input.into_iter().map(|sn| snafu_to_i64(&sn)).sum()),
    );
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input
}

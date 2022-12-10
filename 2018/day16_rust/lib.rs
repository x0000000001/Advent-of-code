use std::fs;

type Instr = Vec<i64>;
type State = Vec<i64>;
struct Test {
    before: State,
    instr: Instr,
    after: State,
}

pub type InputType = (Vec<Test>, Vec<Instr>);

pub fn result_1(input: InputType) -> i64 {
    0
}

pub fn result_2(input: InputType) -> i64 {
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

    let mut tests: Vec<Test> = vec![];
    let mut i = 0;

    let state_from_string = |l: &str| -> Instr {
        l[1..l.len() - 1]
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect()
    };

    let instr_from_string = |l: &str| -> Instr {
        l[1..l.len() - 1]
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect()
    };

    loop {
        let words0 = input[i].split_whitespace().collect::<Vec<&str>>();
        if words0[0] != "Before:" {
            break;
        }
        let words1 = input[i + 2].split_whitespace().collect::<Vec<&str>>();
    }

    todo!()
}

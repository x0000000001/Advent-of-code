use std::fs;

type InputType = Vec<Vec<usize>>;

pub fn part1(s: String) {
    println!("{}", parse(s).iter().filter(|v| is_safe(v)).count());
}

pub fn part2(s: String) {
    println!(
        "{}",
        parse(s)
            .into_iter()
            .filter(|v| {
                is_safe(v)
                    || (0..v.len())
                        .map(|i| {
                            let mut vcopy = v.clone();
                            vcopy.remove(i);
                            is_safe(&vcopy)
                        })
                        .any(|b| b)
            })
            .count()
    );
}

fn is_safe(vec: &Vec<usize>) -> bool {
    vec.len() < 2
        || vec
            .windows(2)
            .all(|w| (1..4).contains(&w[0].abs_diff(w[1])) && ((w[0] < w[1]) == (vec[0] < vec[1])))
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect())
        .collect()
}

fn main() {
    for file in ["data/input_example.txt", "data/input.txt"] {
        let s = fs::read_to_string(file).unwrap();
        part1(s.clone());
        part2(s);
    }
}

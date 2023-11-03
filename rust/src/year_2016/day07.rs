use crate::Solution;

type InputType = Vec<String>;

fn contains_abba(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    for i in 0..(s.len() - 3) {
        if c[i] == c[i + 3] && c[i] != c[i + 1] && c[i + 1] == c[i + 2] {
            return true;
        }
    }

    false
}

pub fn supports_tls(s: &str) -> bool {
    let w = s.replace("]", "[");
    let w: Vec<&str> = w.split("[").collect();
    for i in 0..(w.len() / 2) {
        if contains_abba(w[2 * i + 1]) {
            return false;
        }
    }

    for i in 0..(w.len() / 2 + 1) {
        if contains_abba(w[2 * i]) {
            return true;
        }
    }

    false
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(input.into_iter().filter(|l| supports_tls(l)).count() as i64)
}

pub fn supports_ssl(s: &str) -> bool {
    let w = s.replace("]", "[");
    let w: Vec<&str> = w.split("[").collect();

    let mut abas: Vec<(char, char)> = vec![];
    let mut babs: Vec<(char, char)> = vec![];

    for i in 0..(w.len()) {
        let chars: Vec<char> = w[i].chars().collect();
        for j in 0..(chars.len() - 2) {
            if chars[j] == chars[j + 2] {
                if i % 2 == 0 {
                    abas.push((chars[j], chars[j + 1]));
                } else {
                    babs.push((chars[j], chars[j + 1]));
                }
            }
        }
    }

    for b in babs {
        if abas.contains(&(b.1, b.0)) {
            return true;
        }
    }

    false
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(input.into_iter().filter(|l| supports_ssl(l)).count() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}

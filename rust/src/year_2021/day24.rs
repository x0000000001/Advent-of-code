use crate::Solution;

type InputType = Vec<(i64, i64, i64)>;

#[allow(dead_code)]
fn step(m: i64, n: i64, p: i64, z: i64, w: i64) -> i64 {
    let mut result = z;
    let x = z % 26 + n;
    result /= m;

    if x != w {
        result = result * 26 + w + p;
    }
    result
}

fn possible_inputs(m: i64, n: i64, p: i64, goal: i64, w: i64) -> Vec<i64> {
    let mut v: Vec<i64> = vec![];

    let mut x = goal;
    x = x - w - p;

    if x % 26 == 0 {
        v.push(x / 26 * m);
    }

    if 0 <= w - n && w - n < 26 {
        v.push(w - n + goal * m);
    }

    v
}

fn digit_list_to_num(digits: Vec<u8>) -> i64 {
    let mut i = 0;
    for d in digits {
        i = i * 10 + d as i64;
    }

    i
}

#[allow(dead_code)]
fn test_sol(input: &Vec<(i64, i64, i64)>, i: &i64) -> i64 {
    let digits: Vec<u32> = i
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut z = 0;

    for ((m, n, p), d) in input.iter().zip(digits.iter()) {
        z = step(*m, *n, *p, z, *d as i64);
    }

    z
}

fn get_possible_inputs(input: &mut Vec<(i64, i64, i64)>) -> Vec<i64> {
    input.reverse();

    let mut possible_solutions: Vec<(Vec<u8>, i64)> = Vec::from([(vec![], 0)]);

    for (m, n, p) in input.iter() {
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

    let mut temp: Vec<Vec<u8>> = possible_solutions
        .into_iter()
        .filter(|(_, g)| *g == 0)
        .map(|(l, _)| l)
        .collect();
    for l in &mut temp {
        l.reverse();
    }

    let ps = temp
        .into_iter()
        .map(|l| digit_list_to_num(l))
        .collect::<Vec<i64>>();

    ps
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    Solution::from(get_possible_inputs(&mut input).into_iter().max().unwrap())
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    Solution::from(get_possible_inputs(&mut input).into_iter().min().unwrap())
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    // vec of (m,n,p)
    let mut ns: Vec<(i64, i64, i64)> = vec![];

    for i in 0..input.len() / 18 {
        let m: i64 = input[i * 18 + 4]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let n: i64 = input[i * 18 + 5]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let p: i64 = input[i * 18 + 15]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        ns.push((m, n, p));
    }

    ns
}

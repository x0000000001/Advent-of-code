use crate::Solution;

type InputType = Vec<[i64; 5]>;

pub fn score1(quantities: &Vec<i64>, ing: &Vec<[i64; 5]>) -> i64 {
    let mut s = 1;

    for j in 0..4 {
        let mut temp = 0;
        for i in 0..ing.len() {
            temp += quantities[i] * ing[i][j];
        }
        if temp <= 0 {
            return 0;
        }
        s *= temp;
    }

    s
}
/// Brute-force solution. \
/// I don't like it, but I didn't have the motivation to learn
/// a new constraint solver protocol just for this. \
/// Especially since in Rust I didn't find any really famous/ easy-to-use.
pub fn part1(s: String) -> Solution {
    let ing = parse(s);

    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let l = 100 - i - j - k;
                let score = score1(&Vec::from([i, j, k, l]), &ing);
                if score > max {
                    max = score;
                }
            }
        }
    }

    Solution::from(max)
}

pub fn part2(s: String) -> Solution {
    let ing = parse(s);

    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let l = 100 - i - j - k;
                let score = score1(&Vec::from([i, j, k, l]), &ing);
                let cals = i * ing[0][4] + j * ing[1][4] + k * ing[2][4] + l * ing[3][4];

                if cals != 500 {
                    continue;
                }

                if score > max {
                    max = score;
                }
            }
        }
    }

    Solution::from(max)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = Vec::new();

    for l in input {
        let l = l.replace(",", "");
        let words: Vec<&str> = l.split_whitespace().collect();
        let capacity: i64 = words[2].parse().unwrap();
        let durability: i64 = words[4].parse().unwrap();
        let flavour: i64 = words[6].parse().unwrap();
        let texture: i64 = words[8].parse().unwrap();
        let calories: i64 = words[10].parse().unwrap();

        res.push([capacity, durability, flavour, texture, calories])
    }

    res
}

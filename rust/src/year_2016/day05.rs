use crate::Solution;

type InputType = String;

/// 1 min to run. \
/// Result is not numerical : read output.
pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut n: Vec<char> = vec![];
    let mut count = 0;
    let mut i = 0;

    while count < 8 {
        let digest = md5::compute(format!("{input}{}", i.to_string()));
        let result = format!("{:x}", digest);
        if &result[0..5] == "00000" {
            n.push(result.chars().nth(5).unwrap());
            count += 1;
        }

        i += 1;
    }

    Solution::from(n.iter().collect::<String>())
}

/// 2 min to run. \
/// Result is not numerical : read output.
pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut n: Vec<Option<char>> = vec![None; 8];
    let mut count = 0;
    let mut i = 0;

    while count < 8 {
        let digest = md5::compute(format!("{input}{}", i.to_string()));
        let result = format!("{:x}", digest);
        if &result[0..5] == "00000" {
            if let Some(index) = result.chars().nth(5).unwrap().to_digit(10) {
                if index < 8 && n[index as usize].is_none() {
                    n[index as usize] = Some(result.chars().nth(6).unwrap());
                    count += 1;
                }
            }
        }

        i += 1;
    }

    Solution::from(n.iter().map(|el| el.unwrap()).collect::<String>())
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()
        .remove(0)
}

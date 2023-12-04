mod helpers;

pub use helpers::Solution;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_numbers(l: Vec<char>) -> Vec<Digit> {
    let mut l_nums = vec![];

    for (k, &c) in l.iter().enumerate() {
        if let Some(x) = c.to_digit(10) {
            l_nums.push(Digit::Num(x));
        } else {
            for i in 0..9 {
                let len = NUMS[i].len();
                if k >= len - 1 && l[k + 1 - len..k + 1].iter().collect::<String>() == NUMS[i] {
                    l_nums.push(Digit::Str((i + 1) as u32));
                }
            }
        }
    }

    l_nums
}

enum Digit {
    Num(u32),
    Str(u32),
}

impl Digit {
    fn to_u32(&self) -> u32 {
        match self {
            Digit::Num(x) => *x,
            Digit::Str(x) => *x,
        }
    }
}

type InputType = Vec<Vec<Digit>>;

pub fn part1(s: String) -> Solution {
    Solution::from(
        parse(s)
            .into_iter()
            .map(|digits| {
                digits
                    .into_iter()
                    .filter_map(|d| match d {
                        Digit::Num(x) => Some(x),
                        _ => None,
                    })
                    .collect::<Vec<u32>>()
            })
            .map(|digits| digits[0] * 10 + digits.last().unwrap())
            .sum::<u32>(),
    )
}

pub fn part2(s: String) -> Solution {
    Solution::from(
        parse(s)
            .into_iter()
            .map(|digits| digits[0].to_u32() * 10 + digits.last().unwrap().to_u32())
            .sum::<u32>(),
    )
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|l| get_numbers(l.chars().collect()))
        .collect()
}

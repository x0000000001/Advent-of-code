use crate::Solution;

type InputType = Vec<(u64, u64)>;

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    input.sort_by(|(_, end0), (_, end1)| end0.cmp(end1));
    let mut max_end = 0;

    for (start, end) in input {
        if start < max_end + 2 {
            max_end = end;
        }
    }

    Solution::from(max_end as i64 + 1)
}

pub fn count_free(mut input: InputType, max: u64) -> u64 {
    input.sort_by(|(_, end0), (_, end1)| end0.cmp(end1));
    let mut count = 0;
    let mut max_end: u64 = 0;
    let mut last_free: u64 = 0;

    while max_end < max {
        for (start, end) in input.iter() {
            if *end < max_end {
                continue;
            }

            if *start < max_end + 2 {
                max_end = *end;
                last_free = max;
            } else {
                if last_free >= *start {
                    last_free = start - 1;
                }
            }
        }

        count += last_free - max_end;
        max_end = last_free + 1;
        last_free = max;
    }

    count
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(count_free(input, 4294967295) as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let w = l.split("-").collect::<Vec<&str>>();
        res.push((w[0].parse().unwrap(), w[1].parse().unwrap()));
    }

    res
}

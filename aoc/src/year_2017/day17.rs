use crate::Solution;

type InputType = usize;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut v = vec![];
    v.push(0);
    let mut index = 0;

    for i in 1..2018 {
        index = (index + input) % v.len();
        index += 1;
        v.insert(index, i);
    }

    Solution::from(v[(index + 1) % 2018])
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut value_next_to_0 = 0;
    let mut index = 0;

    for i in 1..50000000 {
        index = (index + input) % i;

        if index == 0 {
            value_next_to_0 = i;
        }

        index += 1;
    }

    Solution::from(value_next_to_0 as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .parse()
        .unwrap()
}

use crate::Solution;

type InputType = Vec<bool>;

fn get_checksum(mut input: InputType, disk_space: usize) -> String {
    while input.len() < disk_space {
        let mut b = input.clone();
        b.reverse();
        for i in 0..b.len() {
            b[i] = !b[i];
        }

        input.push(false);
        input.append(&mut b);
    }

    input.truncate(disk_space);

    while input.len() % 2 == 0 {
        let mut temp = vec![];
        let mut i = 0;
        while i + 1 < input.len() {
            if input[i] == input[i + 1] {
                temp.push(true);
            } else {
                temp.push(false);
            }

            i += 2;
        }

        input = temp;
    }

    let mut s = String::new();

    for b in input {
        s += if b { "1" } else { "0" };
    }

    s += "\n";

    s
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(get_checksum(input, 272))
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(get_checksum(input, 35651584))
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0]
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!(),
        })
        .collect()
}

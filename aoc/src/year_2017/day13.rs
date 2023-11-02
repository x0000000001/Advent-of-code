use crate::Solution;

type InputType = Vec<(u64, u64)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut severity = 0;

    for (depth, range) in input {
        if depth % (2 * (range - 1)) == 0 {
            severity += depth * range;
        }
    }

    Solution::from(severity as i64)
}
/// That's pretty much brute-force... is there a better way ? (missing around with modulos)
pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut i = 0;

    'main_loop: loop {
        for (depth, range) in input.iter() {
            if (depth + i) % (2 * (range - 1)) == 0 {
                i += 1;
                continue 'main_loop;
            }
        }

        break;
    }

    Solution::from(i as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input
        .into_iter()
        .map(|mut l| {
            l = l.replace(":", "");
            let mut w = l.split_whitespace();
            (
                w.next().unwrap().parse().unwrap(),
                w.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

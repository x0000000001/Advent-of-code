use crate::Solution;

type InputType = Vec<(u64, u64)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut i = 0;

    'main_loop: loop {
        let mut time = 1;
        for (modulo, init) in input.iter() {
            if (i + time + init) % modulo != 0 {
                i += 1;
                continue 'main_loop;
            }

            time += 1;
        }

        return Solution::from(i as i64);
    }
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    input.push((11, 0));
    let mut i = 0;

    'main_loop: loop {
        let mut time = 1;
        for (modulo, init) in input.iter() {
            if (i + time + init) % modulo != 0 {
                i += 1;
                continue 'main_loop;
            }

            time += 1;
        }

        return Solution::from(i as i64);
    }
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let l = l.replace(".", "");
        let w = l.split_whitespace().collect::<Vec<&str>>();
        res.push((w[3].parse().unwrap(), w.last().unwrap().parse().unwrap()));
    }

    res
}

use crate::Solution;

type InputType = Vec<usize>;

pub fn part1(s: String) -> Solution {
    let mut cups = parse(s);

    cups = cups.into_iter().map(|el| el - 1).collect();
    let mut nums = (1..10).into_iter().collect::<Vec<usize>>();

    for i in 0..cups.len() - 1 {
        nums[cups[i]] = cups[i + 1];
    }

    nums[cups[cups.len() - 1]] = cups[0];

    let mut current = cups[0];

    for _ in 0..100 {
        let mut removed: Vec<usize> = Vec::new();

        for _ in 0..3 {
            let temp = nums[current];
            nums[current] = nums[temp];
            removed.push(temp);
        }

        let mut insert_index = current;

        while {
            insert_index = if insert_index == 0 {
                8
            } else {
                insert_index - 1
            };

            removed.contains(&insert_index)
        } {}

        let temp = nums[insert_index];
        nums[insert_index] = removed[0];
        nums[removed[2]] = temp;
        current = nums[current];

        // let mut current_temp = 0;
        // for i in 0..9 {
        //     print!("{} ", nums[current_temp] + 1);
        //     current_temp = nums[current_temp];
        // }

        // println!();

        // let mut res = 0;
        // let mut current_temp = 0;
        // for _ in 0..8 {
        //     res = (nums[current_temp] + 1) + res * 10;
        //     current_temp = nums[current_temp];
        // }

        // println!("{res}");
    }

    let mut res = 0;
    let mut current = 0;
    for _ in 0..8 {
        res = (nums[current] + 1) + res * 10;
        current = nums[current];
    }

    Solution::from(res as i64)
}

pub fn part2(s: String) -> Solution {
    let mut cups = parse(s);

    cups = cups.into_iter().map(|el| el - 1).collect();
    let mut nums = (1..1000001).into_iter().collect::<Vec<usize>>();

    for i in 0..cups.len() - 1 {
        nums[cups[i]] = cups[i + 1];
    }

    nums[999999] = cups[0];
    nums[cups[cups.len() - 1]] = cups.len();

    let mut current = cups[0];

    for _ in 0..10000000 {
        let mut removed: Vec<usize> = Vec::new();

        for _ in 0..3 {
            let temp = nums[current];
            nums[current] = nums[temp];
            removed.push(temp);
        }

        let mut insert_index = current;

        while {
            insert_index = if insert_index == 0 {
                999999
            } else {
                insert_index - 1
            };

            removed.contains(&insert_index)
        } {}

        let temp = nums[insert_index];
        nums[insert_index] = removed[0];
        nums[removed[2]] = temp;
        // let x = nums[999999];
        // println!("{}", x);
        current = nums[current];
    }

    Solution::from(((nums[0] + 1) * (nums[nums[0]] + 1)) as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()[0]
        .chars()
        .map(|c| (format!("{c}")).parse().unwrap())
        .collect()
}

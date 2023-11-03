use crate::Solution;

type InputType = (i64, i64, i64, i64);

fn read_numbers(line: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();

    let mut i: usize = 0;
    let mut end_i: usize = 0;

    let mut sign: i64 = 1;

    for c in line.chars() {
        if c.is_digit(10) {
            if i > end_i {
                end_i = i + 1;
            } else {
                end_i += 1;
            }
        } else {
            if i >= end_i {
                i += 1;
            } else {
                nums.push(line[i..end_i].parse::<i64>().unwrap() * sign);
                i = end_i + 1;
            }

            sign = if c == '-' { -1 } else { 1 };
        }
    }

    if end_i > i {
        nums.push(line[i..end_i].parse::<i64>().unwrap() * sign);
    }

    nums
}

// this is a highly stupid solution : brute-force.
// it is as unoptimized as possible
// the problem is symetrical regarding x. therefore, vx0 will always be set positive for solving

// returns : (is_valid, max_y)
fn test_v0(vx0: i64, vy0: i64, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> (bool, i64) {
    let (mut x, mut y) = (0, 0);
    let (mut vx, mut vy) = (vx0, vy0);

    let mut maxy = 0;

    while y >= ymin && x <= xmax {
        if x >= xmin && y <= ymax {
            return (true, maxy);
        }

        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1
        };
        vy -= 1;

        if y > maxy {
            maxy = y;
        }
    }

    (false, 0)
}

pub fn part1(s: String) -> Solution {
    let (xmin, xmax, ymin, ymax) = parse(s);

    let mut max_y_global = 0;

    for vx0 in 0..1000 {
        for vy0 in -1000..1000 {
            let (is_valid, max_y) = test_v0(vx0, vy0, xmin, xmax, ymin, ymax);
            if is_valid && max_y > max_y_global {
                max_y_global = max_y;
            }
        }
    }

    Solution::from(max_y_global)
}

pub fn part2(s: String) -> Solution {
    let (xmin, xmax, ymin, ymax) = parse(s);

    let mut count = 0;

    for vx0 in 0..1000 {
        for vy0 in -10000..10000 {
            let (is_valid, _) = test_v0(vx0, vy0, xmin, xmax, ymin, ymax);
            if is_valid {
                count += 1;
            }
        }
    }

    Solution::from(count)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let nums = read_numbers(&input[0]);

    (nums[0], nums[1], nums[2], nums[3])
}

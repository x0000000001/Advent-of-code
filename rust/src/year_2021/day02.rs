use crate::Solution;

pub fn part1(s: String) -> Solution {
    let mut hoz = 0;
    let mut depth = 0;

    for line in s.lines() {
        let mut els = line.split_whitespace();

        let instruction = els.next().unwrap();
        let amplitude: i32 = els.next().unwrap().parse().unwrap();

        match instruction {
            "forward" => hoz += amplitude,
            "up" => depth -= amplitude,
            "down" => depth += amplitude,
            _ => (),
        }
    }

    Solution::from(hoz * depth)
}

pub fn part2(s: String) -> Solution {
    let mut hoz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in s.lines() {
        let mut els = line.split_whitespace();

        let instruction = els.next().unwrap();
        let amplitude: i32 = els.next().unwrap().parse().unwrap();

        match instruction {
            "forward" => {
                hoz += amplitude;
                depth += aim * amplitude;
            }
            "up" => aim -= amplitude,
            "down" => aim += amplitude,
            _ => (),
        }
    }

    Solution::from(hoz * depth)
}

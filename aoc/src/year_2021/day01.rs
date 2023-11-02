use crate::Solution;

pub fn part1(s: String) -> Solution {
    let mut x = 0;
    let mut temp: i32 = s.lines().next().unwrap().parse().unwrap();

    let mut it = s.lines();
    it.next();

    for line in it {
        let value: i32 = line.parse().unwrap();
        if value > temp {
            x += 1;
        }
        temp = value;
    }

    Solution::from(x)
}

pub fn part2(s: String) -> Solution {
    let mut x = 0;
    let mut vals = [0; 3];

    let mut it = s.lines();

    for i in 0..3 {
        vals[i] = it.next().unwrap().parse().unwrap();
    }

    let mut previous_sum: i32 = vals.iter().sum();

    for line in it {
        for i in 0..2 {
            vals[i] = vals[i + 1];
        }

        vals[2] = line.parse().unwrap();

        let new_sum: i32 = vals.iter().sum();

        if new_sum > previous_sum {
            x += 1;
        }
        previous_sum = new_sum;
    }

    Solution::from(x)
}

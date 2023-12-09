mod helpers;

pub use helpers::Solution;

type InputType = (Vec<usize>, Vec<usize>);

fn equal_points(time: usize, distance: usize) -> (i64, i64) {
    let time_f = time as f32;
    let distance_f = distance as f32;

    let delta_sqrt = ((time_f * time_f) - 4f32 * distance_f).sqrt();

    let mut sol0f = (-time_f + delta_sqrt) / (-2f32);
    let mut sol1f = (-time_f - delta_sqrt) / (-2f32);

    if sol0f > sol1f {
        (sol0f, sol1f) = (sol1f, sol0f);
    }

    let mut sol0 = sol0f.floor() as i64;
    let mut sol1 = sol1f.ceil() as i64;

    // dirty but I don't want to spend time on integer rounding solutio,s
    while (time as i64 - sol0) * sol0 < distance as i64 {
        sol0 += 1;
    }

    while (time as i64 - sol1) * sol1 < distance as i64 {
        sol1 -= 1;
    }

    (sol0, sol1)
}

pub fn part1(s: String) -> Solution {
    let (times, distances) = parse(s);

    Solution::from(
        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| equal_points(time, distance))
            .map(|(sol0, sol1)| sol1 - sol0.max(0) + 1)
            .product::<i64>(),
    )
}

pub fn part2(s: String) -> Solution {
    let (times, distances) = parse(s);

    let merge = |v: Vec<usize>| -> usize {
        v.into_iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    };

    let time = merge(times);
    let distance = merge(distances);
    let (sol0, sol1) = equal_points(time, distance);

    Solution::from(sol1 - sol0 + 1)
}

fn parse(s: String) -> InputType {
    let mut it = s.lines().map(|line| {
        let mut w = line.split_whitespace();
        w.next();

        w.map(|ws| ws.parse::<usize>().unwrap()).collect()
    });

    (it.next().unwrap(), it.next().unwrap())
}

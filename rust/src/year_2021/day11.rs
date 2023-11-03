use crate::Solution;

pub fn part1(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut flashes: i32 = 0;

    let mut octopuses: Vec<Vec<i8>> = input
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let width = octopuses.len();
    let height = octopuses[0].len();

    for _ in 0..100 {
        //increasing everyone's energy by 1
        for i in 0..width {
            for j in 0..height {
                octopuses[i][j] += 1;
            }
        }

        //so that they do not explode 2 times and can be set to 0 afterwise
        let mut explosed_octopuses: Vec<(i8, i8)> = Vec::new();
        //repeating explosions over this list until there are non left to explode
        let mut new_octopuses_over_9: Vec<(i8, i8)> = Vec::new();

        //initialisation of new_octopuses_at_9
        for i in 0..width {
            for j in 0..height {
                if octopuses[i][j] > 9 {
                    new_octopuses_over_9.push((i as i8, j as i8));
                }
            }
        }

        //exploding everyone
        while !new_octopuses_over_9.is_empty() {
            for _ in 0..new_octopuses_over_9.len() {
                let (x, y) = new_octopuses_over_9.pop().unwrap();

                explosed_octopuses.push((x, y));

                //increasing nearby octopuses
                for i in -1..2 {
                    for j in -1..2 {
                        if (x + i) < 0
                            || (x + i) >= width as i8
                            || (y + j) < 0
                            || (y + j) >= height as i8
                        {
                            continue;
                        }
                        //increasing as well octopuses in explosed_octopuses and new_octopuses_at_9, but not a problem...
                        octopuses[(x + i) as usize][(y + j) as usize] += 1;

                        if octopuses[(x + i) as usize][(y + j) as usize] > 9
                            && !explosed_octopuses.contains(&((x + i), (y + j)))
                            && !new_octopuses_over_9.contains(&((x + i), (y + j)))
                        {
                            new_octopuses_over_9.push(((x + i), (y + j)));
                        }
                    }
                }
            }
        }

        flashes += explosed_octopuses.len() as i32;
        for (x, y) in explosed_octopuses {
            octopuses[x as usize][y as usize] = 0;
        }
    }

    Solution::from(flashes)
}

pub fn part2(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut octopuses: Vec<Vec<i8>> = input
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let width = octopuses.len();
    let height = octopuses[0].len();

    let mut step = 0;

    loop {
        step += 1;

        //increasing everyone's energy by 1
        for i in 0..width {
            for j in 0..height {
                octopuses[i][j] += 1;
            }
        }

        //so that they do not explode 2 times and can be set to 0 afterwise
        let mut explosed_octopuses: Vec<(i8, i8)> = Vec::new();
        //repeating explosions over this list until there are non left to explode
        let mut new_octopuses_over_9: Vec<(i8, i8)> = Vec::new();

        //initialisation of new_octopuses_at_9
        for i in 0..width {
            for j in 0..height {
                if octopuses[i][j] > 9 {
                    new_octopuses_over_9.push((i as i8, j as i8));
                }
            }
        }

        //exploding everyone
        while !new_octopuses_over_9.is_empty() {
            for _ in 0..new_octopuses_over_9.len() {
                let (x, y) = new_octopuses_over_9.pop().unwrap();

                explosed_octopuses.push((x, y));

                //increasing nearby octopuses
                for i in -1..2 {
                    for j in -1..2 {
                        if (x + i) < 0
                            || (x + i) >= width as i8
                            || (y + j) < 0
                            || (y + j) >= height as i8
                        {
                            continue;
                        }
                        //increasing as well octopuses in explosed_octopuses and new_octopuses_at_9, but not a problem...
                        octopuses[(x + i) as usize][(y + j) as usize] += 1;

                        if octopuses[(x + i) as usize][(y + j) as usize] > 9
                            && !explosed_octopuses.contains(&((x + i), (y + j)))
                            && !new_octopuses_over_9.contains(&((x + i), (y + j)))
                        {
                            new_octopuses_over_9.push(((x + i), (y + j)));
                        }
                    }
                }
            }
        }

        if explosed_octopuses.len() == width * height {
            break;
        }

        for (x, y) in explosed_octopuses {
            octopuses[x as usize][y as usize] = 0;
        }
    }

    Solution::from(step)
}

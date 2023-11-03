use crate::Solution;

struct Point {
    x: i32,
    y: i32,
}

pub fn part1(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut maxx = 0;
    let mut maxy = 0;

    let lines: Vec<(Point, Point)> = input
        .iter()
        .map(|l| {
            let vals: Vec<&str> = l.split(" -> ").collect();

            let start_vals: Vec<i32> = vals[0].split(',').map(|el| el.parse().unwrap()).collect();
            let end_vals: Vec<i32> = vals[1].split(',').map(|el| el.parse().unwrap()).collect();

            if start_vals[0] > maxx {
                maxx = start_vals[0];
            }
            if start_vals[1] > maxy {
                maxy = start_vals[1];
            }
            if end_vals[0] > maxx {
                maxx = end_vals[0];
            }
            if end_vals[1] > maxy {
                maxy = end_vals[1];
            }

            let start = Point {
                x: start_vals[0],
                y: start_vals[1],
            };

            let end = Point {
                x: end_vals[0],
                y: end_vals[1],
            };

            (start, end)
        })
        .collect();

    // for (start, end) in lines.iter() {
    //     println!("{}, {} -> {}, {}", start.x, start.y, end.x, end.y);
    // }

    //map of lines
    //length +1 since values go from 0 to maxx, 0 to maxy
    let mut occupation: Vec<Vec<i32>> = vec![vec![0; (maxy + 1) as usize]; (maxx + 1) as usize];

    for (start, end) in lines {
        //vertical line
        if start.x == end.x {
            let (min, max) = if start.y > end.y {
                (end.y, start.y + 1)
            } else {
                (start.y, end.y + 1)
            };

            for i in min..max {
                occupation[start.x as usize][i as usize] += 1;
            }
        }
        //horizontal line
        else if start.y == end.y {
            let (min, max) = if start.x > end.x {
                (end.x, start.x + 1)
            } else {
                (start.x, end.x + 1)
            };

            for i in min..max {
                occupation[i as usize][start.y as usize] += 1;
            }
        }
    }

    let mut count = 0;

    for i in 0..maxx + 1 {
        for j in 0..maxy + 1 {
            //print!("{} ", occupation[i as usize][j as usize]);
            if occupation[i as usize][j as usize] >= 2 {
                count += 1;
            }
        }
        //println!();
    }

    Solution::from(count)
}

pub fn part2(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut maxx = 0;
    let mut maxy = 0;

    let lines: Vec<(Point, Point)> = input
        .iter()
        .map(|l| {
            let vals: Vec<&str> = l.split(" -> ").collect();

            let start_vals: Vec<i32> = vals[0].split(',').map(|el| el.parse().unwrap()).collect();
            let end_vals: Vec<i32> = vals[1].split(',').map(|el| el.parse().unwrap()).collect();

            if start_vals[0] > maxx {
                maxx = start_vals[0];
            }
            if start_vals[1] > maxy {
                maxy = start_vals[1];
            }
            if end_vals[0] > maxx {
                maxx = end_vals[0];
            }
            if end_vals[1] > maxy {
                maxy = end_vals[1];
            }

            let start = Point {
                x: start_vals[0],
                y: start_vals[1],
            };

            let end = Point {
                x: end_vals[0],
                y: end_vals[1],
            };

            (start, end)
        })
        .collect();

    // for (start, end) in lines.iter() {
    //     println!("{}, {} -> {}, {}", start.x, start.y, end.x, end.y);
    // }

    //map of lines
    //length +1 since values go from 0 to maxx, 0 to maxy
    let mut occupation: Vec<Vec<i32>> = vec![vec![0; (maxy + 1) as usize]; (maxx + 1) as usize];

    for (start, end) in lines {
        //vertical line
        if start.x == end.x {
            let (min, max) = if start.y > end.y {
                (end.y, start.y + 1)
            } else {
                (start.y, end.y + 1)
            };

            for i in min..max {
                occupation[start.x as usize][i as usize] += 1;
            }
        }
        //horizontal line
        else if start.y == end.y {
            let (min, max) = if start.x > end.x {
                (end.x, start.x + 1)
            } else {
                (start.x, end.x + 1)
            };

            for i in min..max {
                occupation[i as usize][start.y as usize] += 1;
            }
        }
        //diagonal lines
        else {
            let sensx = if start.x > end.x { -1 } else { 1 };

            let sensy = if start.y > end.y { -1 } else { 1 };

            let length = if start.y > end.y {
                start.y - end.y + 1
            } else {
                end.y - start.y + 1
            };

            for i in 0..length {
                occupation[(start.x + i * sensx) as usize][(start.y + i * sensy) as usize] += 1;
            }
        }
    }

    let mut count = 0;

    for i in 0..maxx + 1 {
        for j in 0..maxy + 1 {
            //print!("{} ", occupation[i as usize][j as usize]);
            if occupation[i as usize][j as usize] >= 2 {
                count += 1;
            }
        }
        //println!();
    }

    Solution::from(count)
}

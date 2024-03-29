use crate::Solution;

type InputType = Vec<Vec<char>>;

// word, nb of steps
fn go(input: &InputType) -> (String, u64) {
    let mut letters_seen: Vec<char> = vec![];
    let (mut x, mut y): (i64, i64) = (0, 0);
    // 0 : right, 1 : down, 2 : left, 3 : up
    let mut direction = 1;
    let mut steps_count = 0;

    for i in 0..input[0].len() {
        if input[0][i] == '|' {
            x = i as i64;
            break;
        }
    }

    loop {
        let (mut nextx, mut nexty): (i64, i64) = (x, y);

        match direction {
            0 => nextx += 1,
            1 => nexty += 1,
            2 => nextx -= 1,
            3 => nexty -= 1,
            _ => panic!(),
        }

        if y < 0 || y >= input.len() as i64 || x < 0 || x >= input[0].len() as i64 {
            break;
        }

        let c = input[y as usize][x as usize];

        // println!("{x} {y} {c}");
        if c == ' ' {
            break;
        }

        match input[y as usize][x as usize] {
            '|' | '-' => {
                x = nextx;
                y = nexty;
            }
            '+' => {
                for d in 0..4 {
                    // not going back
                    if d == (direction + 2) % 4 {
                        continue;
                    }

                    let (mut candidate_x, mut candidate_y): (i64, i64) = (x, y);

                    match d {
                        0 => candidate_x += 1,
                        1 => candidate_y += 1,
                        2 => candidate_x -= 1,
                        3 => candidate_y -= 1,
                        _ => panic!(),
                    }

                    if candidate_x >= 0
                        && candidate_x < input[0].len() as i64
                        && candidate_y >= 0
                        && candidate_y < input.len() as i64
                    {
                        let candidate_c = input[candidate_y as usize][candidate_x as usize];
                        if candidate_c.is_alphabetic() || // letter at intersection
                         ([0,2].contains(&d) && (candidate_c == '-')) ||  // hoz move
                         ([1,3].contains(&d) && (candidate_c == '|'))
                        {
                            // vert move
                            direction = d;
                            x = candidate_x;
                            y = candidate_y;
                        }
                    }
                }
            }
            _ => {
                letters_seen.push(c);
                x = nextx;
                y = nexty;
            }
        }

        steps_count += 1;
    }

    let word: String = letters_seen.iter().collect();

    (word, steps_count)
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(go(&input).0)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(go(&input).1 as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|l| l.to_owned())
        .map(|l| l.chars().collect())
        .collect()
}

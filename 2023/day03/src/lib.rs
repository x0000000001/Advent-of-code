mod helpers;

pub use helpers::Solution;

type InputType = Vec<Vec<char>>;

pub fn part1(s: String) -> Solution {
    let lines = parse(s);

    let mut numbers = vec![];
    let h = lines.len();
    let w = lines[0].len();

    for i in 0..h {
        let mut current_number = vec![];

        for j in 0..(w + 1) {
            let c = if j == w { '_' } else { lines[i][j] };

            if c.is_numeric() {
                current_number.push(c);
            } else {
                let l = current_number.len();

                if l != 0 {
                    numbers.push((
                        i,
                        j - l,
                        j - 1,
                        current_number
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    ));
                }

                current_number.clear();
            }
        }
    }

    let mut sum = 0;

    for (i_line, j_begin, j_end, num) in numbers {
        let mut indices = vec![];

        for i in [i_line as i64 - 1, i_line as i64 + 1] {
            for j in (j_begin as i64 - 1)..(j_end as i64 + 2) {
                if i >= 0 && i < h as i64 && j >= 0 && j < w as i64 {
                    indices.push((i as usize, j as usize));
                }
            }
        }

        if j_begin > 0 {
            indices.push((i_line, j_begin - 1));
        }
        if j_end < w - 1 {
            indices.push((i_line, j_end + 1));
        }

        if indices
            .into_iter()
            .map(|(i, j)| !lines[i][j].is_numeric() && lines[i][j] != '.')
            .fold(false, |b_s, b| b || b_s)
        {
            sum += num;
        }
    }

    Solution::from(sum as u64)
}

pub fn part2(s: String) -> Solution {
    let lines = parse(s);

    let mut sum = 0;
    let h = lines.len();
    let w = lines[0].len();

    for i in 0..h {
        for j in 0..w {
            if lines[i][j] != '*' {
                continue;
            }

            let i_64 = i as i64;
            let j_64 = j as i64;

            let mut neighbor_numbers = ((i_64 - 1)..(i_64 + 2))
                .map(|i| {
                    (j_64 - 1..j_64 + 2).filter_map(move |j| {
                        if i >= 0 && i < h as i64 && j >= 0 && j < w as i64 {
                            Some((i as usize, j as usize))
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .filter_map(|(i, j)| {
                    if lines[i][j].is_numeric() {
                        let mut begin_index = j;

                        while begin_index > 0 {
                            if lines[i][begin_index - 1].is_numeric() {
                                begin_index -= 1;
                            } else {
                                break;
                            }
                        }

                        let mut end_index = j;

                        while end_index < w - 1 {
                            if lines[i][end_index + 1].is_numeric() {
                                end_index += 1;
                            } else {
                                break;
                            }
                        }

                        Some(
                            lines[i][begin_index..(end_index + 1)]
                                .iter()
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            neighbor_numbers.sort();
            neighbor_numbers.dedup();

            match neighbor_numbers.len() {
                0 | 1 => (),
                2 => sum += neighbor_numbers.iter().product::<usize>(),
                _ => panic!(),
            }
        }
    }

    Solution::from(sum as u64)
}

fn parse(s: String) -> InputType {
    s.lines().map(|s| s.chars().collect()).collect()
}

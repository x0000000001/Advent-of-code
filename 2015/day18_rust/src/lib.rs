pub type InputType = Vec<Vec<bool>>;

fn step(input: &mut Vec<Vec<bool>>) {
    let (w, h) = (input.len(), input[0].len());
    let mut temp = input.clone();

    for i in 0..w {
        for j in 0..h {
            let mut neighbours = 0;
            for newi in [i as isize - 1, i as isize, i as isize + 1] {
                for newj in [j as isize - 1, j as isize, j as isize + 1] {
                    if newi >= w as isize || newi < 0 || newj >= h as isize || newj < 0 {
                        continue;
                    }
                    if input[newi as usize][newj as usize] {
                        neighbours += 1;
                    }
                }
            }

            if input[i][j] {
                neighbours -= 1;
            }

            if input[i][j] {
                if neighbours != 2 && neighbours != 3 {
                    temp[i][j] = false;
                }
            } else {
                if neighbours == 3 {
                    temp[i][j] = true;
                }
            }
        }
    }

    for i in 0..w {
        for j in 0..h {
            input[i][j] = temp[i][j];
        }
    }
}

pub fn count_after_step(
    mut input: &mut InputType,
    steps_count: usize,
    are_corners_on: bool,
) -> u64 {
    let (w, h) = (input.len(), input[0].len());

    if are_corners_on {
        input[0][0] = true;
        input[w - 1][0] = true;
        input[0][h - 1] = true;
        input[w - 1][h - 1] = true;
    }

    for _ in 0..steps_count {
        step(&mut input);
        if are_corners_on {
            input[0][0] = true;
            input[w - 1][0] = true;
            input[0][h - 1] = true;
            input[w - 1][h - 1] = true;
        }
    }

    let mut count = 0;

    for i in 0..w {
        for j in 0..h {
            if input[i][j] {
                count += 1;
            }
        }
    }

    count
}

pub fn result_1(mut input: InputType) -> i64 {
    count_after_step(&mut input, 100, false) as i64
}

pub fn result_2(mut input: InputType) -> i64 {
    count_after_step(&mut input, 100, true) as i64
}

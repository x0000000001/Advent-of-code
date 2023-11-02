pub type InputType = Vec<u32>;

pub fn step(input: &mut Vec<u32>) {
    let mut temp: InputType = vec![];

    let mut current_num = input[0];
    let mut current_count = 0;

    for j in 0..input.len() {
        if input[j] == current_num {
            current_count += 1;
        } else {
            temp.push(current_count);
            temp.push(current_num);
            current_count = 1;
            current_num = input[j];
        }
    }

    temp.push(current_count);
    temp.push(current_num);

    input.clear();
    input.append(&mut temp);
}

pub fn result_1(mut input: InputType) -> i64 {
    for _ in 0..40 {
        step(&mut input);
    }

    input.len() as i64
}

/// hmm... well the naïve way works as well for problem 2... \
/// in < 2s... \
/// so hhmm... I guess that's done ?
pub fn result_2(mut input: InputType) -> i64 {
    for _ in 0..50 {
        step(&mut input);
    }

    input.len() as i64
}

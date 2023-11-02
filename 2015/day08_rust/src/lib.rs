pub type InputType = Vec<String>;

pub fn count_real_chars(s: &str) -> i64 {
    let mut i = 0;
    let mut count = 0;
    let len = s.len();

    let c = s.chars().collect::<Vec<char>>();

    while i < len {
        match c[i] {
            '\\' => match c[i + 1] {
                'x' => i += 4,
                _ => i += 2,
            },
            _ => i += 1,
        }

        count += 1;
    }

    count
}

pub fn result_1(input: InputType) -> i64 {
    let real_chars_count: i64 = input
        .iter()
        .map(|l| count_real_chars(&l[1..(l.len() - 1)]))
        .sum();
    let writted_chars_count: i64 = input.iter().map(|l| l.chars().count() as i64).sum();
    writted_chars_count - real_chars_count
}

pub fn count_added_chars(s: &str) -> i64 {
    let mut i = 0;
    let mut count = 0;
    let len = s.len();

    let c = s.chars().collect::<Vec<char>>();

    while i < len {
        match c[i] {
            '\\' => count += 2,
            '\"' => count += 2,
            _ => count += 1,
        }
        i += 1;
    }

    count + 2 // +2 for quotation marks
}

pub fn result_2(input: InputType) -> i64 {
    let writted_chars_count: i64 = input.iter().map(|l| l.chars().count() as i64).sum();
    let added_chars_count: i64 = input.iter().map(|l| count_added_chars(&l)).sum();
    added_chars_count - writted_chars_count
}

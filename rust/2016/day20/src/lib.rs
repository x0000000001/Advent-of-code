pub type InputType = Vec<(u64,u64)>;

pub fn result_1(mut input: InputType) -> i64
{
    input.sort_by(|(_,end0),(_,end1)| end0.cmp(end1));
    let mut max_end = 0;

    for (start,end) in input {
        if start < max_end+2 {
            max_end = end;
        }
    }
    max_end as i64 + 1
}

pub fn count_free(mut input: InputType, max: u64) -> u64 {

    input.sort_by(|(_,end0),(_,end1)| end0.cmp(end1));
    let mut count = 0;
    let mut max_end: u64 = 0;
    let mut last_free: u64 = 0;

    while max_end < max {

        for (start,end) in input.iter() {
            if *end < max_end {
                continue;
            }

            if *start < max_end+2 {
                max_end = *end;
                last_free = max;
            } else {
                if last_free >= *start {
                    last_free = start-1;
                }
            }
        }

        count += last_free-max_end;
        max_end = last_free+1;
        last_free = max;
    }

    count
}

// 203 too high
pub fn result_2(input: InputType) -> i64
{   
    count_free(input, 4294967295) as i64
}
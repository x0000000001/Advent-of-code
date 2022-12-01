pub type InputType = Vec<char>;

fn unzip(s: &Vec<char>) -> Vec<char> {
    let mut res: Vec<char> = vec![];
    let len = s.len();
    let mut i = 0;

    while i < len {
        if s[i] == '(' {
            let mut j = i+1;
            let mut n0 = 0;
            while let Some(d) = s[j].to_digit(10) {
                n0 = n0 * 10 + d;
                j += 1;
            }
            if s[j] == 'x' {
                j += 1;
                let mut n1 = 0;
                while let Some(d) = s[j].to_digit(10) {
                    n1 = n1 * 10 + d;
                    j += 1;
                }
                if s[j] == ')' {
                    // copy happens here
                    for _ in 0..n1 {
                        for k in 0..n0 as usize {
                            res.push(s[j+k+1]);
                        }
                    }
                    i = j+1+n0 as usize;
                    continue;
                }
            }

        } 

        res.push(s[i]);
        i += 1;
    }

    res
}

pub fn result_1(input: InputType) -> i64
{
    let res = unzip(&input);
    res.len() as i64
}

/// recursive
fn count_final(s: &[char]) -> usize {
    let len = s.len();
    let mut i = 0;
    let mut count = 0;

    while i < len {
        if s[i] == '(' {
            let mut j = i+1;
            let mut n0 = 0;
            while let Some(d) = s[j].to_digit(10) {
                n0 = n0 * 10 + d;
                j += 1;
            }
            if s[j] == 'x' {
                j += 1;
                let mut n1 = 0;
                while let Some(d) = s[j].to_digit(10) {
                    n1 = n1 * 10 + d;
                    j += 1;
                }
                if s[j] == ')' {
                    // copy happens here
                    count += count_final(&s[(j+1)..(j+1+n0 as usize)]) * n1 as usize;
                    i = j+1+n0 as usize;
                    continue;
                }
            }

        } 

        count += 1;
        i += 1;
    }

    count
}

pub fn result_2(input: InputType) -> i64
{   
    count_final(&input) as i64
}
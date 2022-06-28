use std::fs;

pub fn result_1() -> i32
{  
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let bit_words_len = contents.lines().next().unwrap().chars().count();
    let mut lines_count = 0;

    let mut bits_count = vec![0;bit_words_len];

    for line in contents.lines(){

        let mut i = 0;

        for bit in line.chars(){
            let val = bit.to_digit(10).unwrap();

            if val == 1{
                bits_count[i] += 1
            }

            i+=1;
        }

        lines_count +=1;
    }

    let base:i32 = 2;
    let mut factor = base.pow(bit_words_len as u32 - 1);

    for i in 0..bit_words_len{
        if bits_count[i] > (lines_count/2){
            gamma_rate += factor; 
        }else {
            epsilon_rate += factor; 
        }

        factor /= 2;
    }


    gamma_rate * epsilon_rate
}

pub fn result_2() -> usize
{   
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    select_candidate(&input, '1') * select_candidate(&input, '0')
}

fn select_candidate(_candidates: &Vec<&str>, criteria_bit: char) -> usize
{
    let mut candidates = _candidates.clone();
    //first oxygen choice
    let mut bit = 0;
    while candidates.len() > 1 {
        let bit_count = 
        candidates.iter()
            .filter(|candidate| candidate.chars().nth(bit).unwrap() == '1').count();

        let preponderant_bit = match criteria_bit {
            '1' => {
                if 2 * bit_count >= candidates.len(){
                    '1'
                }else {
                    '0'
                }
            },
            '0' => {
                if 2 * bit_count >= candidates.len() {
                    '0'
                } else {
                    '1'
                }
            },
            _ => '1'
        };

        candidates.retain(|candidate| candidate.chars().nth(bit).unwrap() == preponderant_bit);
        bit += 1;
    }

    let binary = candidates.get(0).unwrap();
    let mut n: usize = 0;

    for i in 0..binary.chars().count() {
        n = 2 * n + binary.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
    }
    n
}
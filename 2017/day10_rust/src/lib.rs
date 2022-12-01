use std::fs;

pub type InputType = String;

pub fn result_1(input: InputType) -> i64
{
    let nums: Vec<usize> = 
        input.split(",")
        .map(|el| el.parse().unwrap()).collect();
    
    let len = 256;

    let mut l: Vec<usize> = (0..len).collect();
    let mut skip_size = 0;
    let mut start_offset = 0;

    for length in nums {

        if length > len {
            continue;
        }

        l[0..length].reverse();
        l.rotate_left((length + skip_size)%len);
        start_offset += len - (length + skip_size)%len;
        start_offset %= len;

        skip_size += 1;
    }

    
    (l[start_offset] * l[(start_offset+1)%len]) as i64
}


pub fn result_2(input: InputType) -> i64
{   
    let mut ascci_codes: Vec<usize> = input.chars().map(|c| {
        let mut buffer = [0;1];
        c.encode_utf8(&mut buffer);
        buffer[0] as usize
    }
    ).collect();

    ascci_codes.push(17);
    ascci_codes.push(31);
    ascci_codes.push(73);
    ascci_codes.push(47);
    ascci_codes.push(23);

    let nums = ascci_codes.repeat(64);
    
    let len = 256;

    let mut l: Vec<usize> = (0..len).collect();
    let mut skip_size = 0;
    let mut start_offset = 0;

    for length in nums {

        if length > len {
            continue;
        }

        l[0..length].reverse();
        l.rotate_left((length + skip_size)%len);
        start_offset += len - (length + skip_size)%len;
        start_offset %= len;

        skip_size += 1;
    }

    l.rotate_left(start_offset);

    let mut xord: Vec<u8> = vec![0;16];

    for i in 0..16 {
        xord[i] = l[i*16] as u8;
        for j in 1..16 {
            xord[i] ^= l[i*16 + j] as u8;
        }
    }
    
    for x in xord.iter() {
        let res = format!("{:x}",x);
        if res.len() == 1 {
            print!("0");
        }
        print!("{res}");
    }
    println!();

    0
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let mut input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    input.pop().unwrap()
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;
    
    #[test]
    fn test2()
    {
        assert_eq!(result_2("".to_string()), 0);
    }
}
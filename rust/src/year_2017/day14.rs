use crate::Solution;

type InputType = String;

fn hex_to_bits(c: &char) -> Vec<bool> {
    match c {
        '0' => Vec::from([false, false, false, false]),
        '1' => Vec::from([false, false, false, true]),
        '2' => Vec::from([false, false, true, false]),
        '3' => Vec::from([false, false, true, true]),
        '4' => Vec::from([false, true, false, false]),
        '5' => Vec::from([false, true, false, true]),
        '6' => Vec::from([false, true, true, false]),
        '7' => Vec::from([false, true, true, true]),
        '8' => Vec::from([true, false, false, false]),
        '9' => Vec::from([true, false, false, true]),
        'a' => Vec::from([true, false, true, false]),
        'b' => Vec::from([true, false, true, true]),
        'c' => Vec::from([true, true, false, false]),
        'd' => Vec::from([true, true, false, true]),
        'e' => Vec::from([true, true, true, false]),
        'f' => Vec::from([true, true, true, true]),
        _ => panic!(),
    }
}

fn string_to_bits(s: &Vec<char>) -> Vec<bool> {
    s.iter().flat_map(|c| hex_to_bits(&c)).collect()
}

fn knot_hash(s: &str) -> Vec<char> {
    let mut ascci_codes: Vec<usize> = s
        .chars()
        .map(|c| {
            let mut buffer = [0; 1];
            c.encode_utf8(&mut buffer);
            buffer[0] as usize
        })
        .collect();

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
        l.rotate_left((length + skip_size) % len);
        start_offset += len - (length + skip_size) % len;
        start_offset %= len;

        skip_size += 1;
    }

    l.rotate_left(start_offset);

    let mut xord: Vec<u8> = vec![0; 16];

    for i in 0..16 {
        xord[i] = l[i * 16] as u8;
        for j in 1..16 {
            xord[i] ^= l[i * 16 + j] as u8;
        }
    }

    let mut result = vec![];

    for x in xord.iter() {
        let res = format!("{:x}", x);
        if res.len() == 1 {
            result.push('0');
            result.push(res.chars().next().unwrap());
        } else {
            let mut it = res.chars();
            result.push(it.next().unwrap());
            result.push(it.next().unwrap());
        }
    }

    result
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        (0..128)
            .flat_map(|i| string_to_bits(&knot_hash(&format!("{}-{}", input, i))))
            .filter(|b| *b)
            .count() as i64,
    )
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let grid = (0..128)
        .map(|i| string_to_bits(&knot_hash(&format!("{}-{}", input, i))))
        .collect::<Vec<Vec<bool>>>();

    let mut seen: Vec<Vec<bool>> = vec![vec![false; 128]; 128];
    let mut regions_count = 0;

    for i in 0..128 {
        for j in 0..128 {
            if seen[i][j] || !grid[i][j] {
                continue;
            }

            regions_count += 1;

            let mut queue: Vec<(usize, usize)> = vec![];
            queue.push((i, j));
            seen[i][j] = true;

            while !queue.is_empty() {
                let (currenti, currentj) = queue.pop().unwrap();

                let mut candidates = vec![];
                if currenti > 0 {
                    candidates.push((currenti - 1, currentj));
                }
                if currenti < 127 {
                    candidates.push((currenti + 1, currentj));
                }
                if currentj > 0 {
                    candidates.push((currenti, currentj - 1));
                }
                if currentj < 127 {
                    candidates.push((currenti, currentj + 1));
                }

                for c in candidates {
                    if !seen[c.0][c.1] && grid[c.0][c.1] {
                        seen[c.0][c.1] = true;
                        queue.push(c);
                    }
                }
            }
        }
    }

    Solution::from(regions_count)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .to_owned()
}

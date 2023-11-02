pub type InputType = Vec<bool>;

fn print_checksum(mut input: InputType, disk_space: usize) {
    while input.len() < disk_space {
        let mut b = input.clone();
        b.reverse();
        for i in 0..b.len() {
            b[i] = !b[i];
        }

        input.push(false);
        input.append(&mut b);
    }

    input.truncate(disk_space);

    while input.len() % 2 == 0 {
        let mut temp = vec![];
        let mut i = 0;
        while i + 1 < input.len() {
            if input[i] == input[i + 1] {
                temp.push(true);
            } else {
                temp.push(false);
            }

            i += 2;
        }

        input = temp;
    }

    for b in input {
        print!("{}", if b { 1 } else { 0 });
    }
    println!();
}

pub fn result_1(input: InputType) -> i64 {
    print_checksum(input, 272);
    0
}

pub fn result_2(input: InputType) -> i64 {
    print_checksum(input, 35651584);
    0
}

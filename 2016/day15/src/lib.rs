// nb of positions, initial position
pub type InputType = Vec<(u64,u64)>;

pub fn result_1(input: InputType) -> i64
{
    let mut i = 0;

    'main_loop: loop {
        let mut time = 1;
        for (modulo,init) in input.iter() {
            if (i + time + init)%modulo != 0 {
                i += 1;
                continue 'main_loop;
            }

            time += 1;
        }

        return i as i64;
    }
}


pub fn result_2(mut input: InputType) -> i64
{   
    input.push((11,0));
    let mut i = 0;

    'main_loop: loop {
        let mut time = 1;
        for (modulo,init) in input.iter() {
            if (i + time + init)%modulo != 0 {
                i += 1;
                continue 'main_loop;
            }

            time += 1;
        }

        return i as i64;
    }
}
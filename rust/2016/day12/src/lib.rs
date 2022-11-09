// cpy : 0
// inc : 1
// dec : 2
// jnz : 3
// a,b,c,d : 0,1,2,3
pub type InputType = Vec<(u64,InstrVal,InstrVal)>;

#[derive(Clone, Copy)]
pub enum InstrVal {
    Val(i64),
    Index(usize),
    None
}

fn run_instr(reg: &mut[i64], instructions: &InputType) {
    let mut index = 0;

    'instr_loop: while index < instructions.len() {
        let (instr,x,y) = instructions[index];

        match instr {
            0 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => panic!("Can't copy wrong value.")
                };

                match y {
                    InstrVal::Index(i) => reg[i] = val,
                    _ => panic!("Wrong register to copy in.")
                }
            },

            1 => {
                match x {
                    InstrVal::Index(i) => reg[i] += 1,
                    _ => panic!("Wrong register to increment.")
                }
            },

            2 => {
                match x {
                    InstrVal::Index(i) => reg[i] -= 1,
                    _ => panic!("Wrong register to increment.")
                }
            },

            3 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => panic!("Can't copy wrong value.")
                };

                if val == 0 {
                    index += 1;
                    continue 'instr_loop;
                }

                match y {
                    InstrVal::Val(i) => {
                        index = (index as i64 + i) as usize;
                        continue 'instr_loop;
                    },
                    _ => panic!("Wrong register to increment.")
                }
            },

            _ => panic!("Unknown instruction : {}", instr)
        }

        index += 1;
    } 
}

pub fn result_1(input: InputType) -> i64
{
    let mut reg = [0,0,0,0];
    run_instr(&mut reg, &input);
    reg[0]
}


pub fn result_2(input: InputType) -> i64
{   
    let mut reg = [0,0,1,0];
    run_instr(&mut reg, &input);
    reg[0]
}
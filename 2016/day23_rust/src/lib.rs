// cpy : 0
// inc : 1
// dec : 2
// jnz : 3
// tgl : 4
// a,b,c,d : 0,1,2,3
pub type InputType = Vec<(u64,InstrVal,InstrVal)>;

#[derive(Clone, Copy)]
pub enum InstrVal {
    Val(i64),
    Index(usize),
    None
}

fn run_instr(reg: &mut[i64], mut instructions: InputType) {
    let mut index = 0;

    'instr_loop: while index < instructions.len() {
        let (instr,x,y) = instructions[index];
        // println!("{instr} {:?}",reg);

        match instr {
            0 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => {
                        index += 1;
                        continue 'instr_loop;
                    }
                };

                match y {
                    InstrVal::Index(i) => reg[i] = val,
                    _ => {
                        index += 1;
                        continue 'instr_loop;
                    }
                }
            },

            1 => {
                match x {
                    InstrVal::Index(i) => reg[i] += 1,
                    _ => {
                        index += 1;
                        continue 'instr_loop;
                    }
                }
            },

            2 => {
                match x {
                    InstrVal::Index(i) => reg[i] -= 1,
                    _ => {
                        index += 1;
                        continue 'instr_loop;
                    }
                }
            },

            3 => {
                let val = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => {
                        index += 1;
                        continue 'instr_loop;
                    }
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
                    InstrVal::Index(i) => {
                        index = (index as i64 + reg[i]) as usize;
                        continue 'instr_loop;
                    },
                    _ => {
                        index += 1;
                        continue 'instr_loop;
                    }
                }
            },

            4 => {
                let i = match x {
                    InstrVal::Val(v) => v,
                    InstrVal::Index(i) => reg[i],
                    InstrVal::None => {
                        index += 1;
                        continue 'instr_loop;
                    }
                } as usize + index;

                if i >= instructions.len() {
                    index += 1;
                    continue 'instr_loop;
                }

                let (instr1,x1,y1) = instructions[i];

                instructions[i] = match instr1 {
                    1 => (2,x1,y1),
                    2 => (1,x1,y1),
                    4 => (1,x1,y1),
                    3 => (0,x1,y1),
                    0 => (3,x1,y1),
                    _ => panic!()
                }
            }

            _ => panic!("Unknown instruction : {}", instr)
        }

        index += 1;
    } 
}

pub fn result_1(input: InputType) -> i64
{
    let mut reg = [7,0,0,0];
    run_instr(&mut reg, input);
    reg[0]
}

/// 182s to run : this is naïve solution. \
/// Possible optimization : understand how multiplication is done. \
/// But for now I'm lazy.
pub fn result_2(input: InputType) -> i64
{   
    let mut reg = [12,0,0,0];
    run_instr(&mut reg, input);
    reg[0]
}
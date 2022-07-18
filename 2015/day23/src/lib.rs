// (instruction, arg, arg)
// a = 0, b = 1
// hlf = 0, tpl = 1, inc = 2, jmp = 3,jie = 4, jio = 5
pub type InputType = Vec<(i64,i64,i64)>;

pub fn result_1(instructions: InputType) -> i64
{
    let mut i = 0;
    let mut var = [0,0];

    while i < instructions.len(){
        let (instr, arg0, arg1) = instructions[i];
        match instr {
            0 => var[arg0 as usize] /= 2,
            1 => var[arg0 as usize] *= 3,
            2 => var[arg0 as usize] += 1,
            3 => {
                i = (i as i64 + arg0) as usize;
                continue;
            },
            4 => {
                if var[arg0 as usize]%2 == 0 {
                    i += arg1 as usize;
                    continue;
                }
            },
            5 => {
                if var[arg0 as usize] == 1 {
                    i += arg1 as usize;
                    continue;
                }
            },
            _ => ()
        }

        i += 1;
    }

    var[1]
}


pub fn result_2(instructions: InputType) -> i64
{   
    let mut i = 0;
    let mut var = [1,0];

    while i < instructions.len(){
        let (instr, arg0, arg1) = instructions[i];
        match instr {
            0 => var[arg0 as usize] /= 2,
            1 => var[arg0 as usize] *= 3,
            2 => var[arg0 as usize] += 1,
            3 => {
                i = (i as i64 + arg0) as usize;
                continue;
            },
            4 => {
                if var[arg0 as usize]%2 == 0 {
                    i += arg1 as usize;
                    continue;
                }
            },
            5 => {
                if var[arg0 as usize] == 1 {
                    i += arg1 as usize;
                    continue;
                }
            },
            _ => ()
        }

        i += 1;
    }

    var[1]
}
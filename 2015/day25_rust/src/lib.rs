pub type InputType = (i64,i64);

pub fn result_1((x,y): InputType) -> i64
{
    let mut code = 20151125;
    let factor = 252533;
    let modulo = 33554393;

    let n = ((x+y)*(x+y-1))/2 - x + 1;

    for _ in 1..n {
        code *= factor;
        code %= modulo;
    }

    code
}


pub fn result_2(_input: InputType) -> i64
{   
    0
}
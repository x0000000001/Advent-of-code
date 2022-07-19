pub type InputType = Vec<Vec<char>>;

const KEYS: &[((i64,i64),i64)] = &[((-1,1),1),((0,1),2),((1,1),3),((-1,0),4),((0,0),5),((1,0),6),((-1,-1),7),((0,-1),8),((1,-1),9)];

pub fn result_1(input: InputType) -> i64
{
    let mut pos = (0,0);
    let mut code = 0;

    for l in input {
        for c in l {
            match c {
                'U' => pos.1 = (pos.1 + 1).min(1),
                'D' => pos.1 = (pos.1-1).max(-1),
                'L' => pos.0 = (pos.0-1).max(-1),
                _ => pos.0 = (pos.0+1).min(1)
            }
        }

        for (p,n) in KEYS.iter() {
            if *p == pos {
                code = code * 10 + n;
            }
        }
    }

    code
}

const KEYS_1: &[((i64,i64),char)] = &[((-1,1),'2'),((0,1),'3'),((1,1),'4'),
((-1,0),'6'),((0,0),'7'),((1,0),'8'),((-1,-1),'A'),((0,2),'1'),((-2,0),'5'),((2,0),'9'),
((0,-1),'B'),((1,-1),'C'),((0,-2),'D')];

/// for this puzzle, since te answer is not numerical, read what's printed
pub fn result_2(input: InputType) -> i64
{   
    let mut pos = (-2,0);
    let mut code = vec![];

    for l in input {
        for c in l {
            let mut temp = pos;
            match c {
                'U' => temp.1 += 1,
                'D' => temp.1 -= 1,
                'L' => temp.0 -= 1,
                _ => temp.0 += 1
            }
            for (p,_) in KEYS_1.iter() {
                if *p == temp {
                    pos = temp;
                    break;
                }
            }
        }
        for (p,c) in KEYS_1.iter() {
            if *p == pos {
                code.push(c);
                break;
            }
        }
    }

    println!("{:?}", code);
    0
}
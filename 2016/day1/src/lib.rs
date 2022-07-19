pub type InputType = Vec<(char, i64)>;

pub fn result_1(input: InputType) -> i64
{
    let mut orientation = 0;
    let mut position = (0,0);

    for (d,i) in input {
        if d == 'R' {
            orientation = (orientation+1)%4;
        } else {
            orientation = (orientation-1)%4;
            if orientation < 0 {
                orientation = 3;
            }
        }

        match orientation {
            0 => position.1 += i,
            1 => position.0 += i,
            2 => position.1 -= i,
            3 => position.0 -= i,
            _ => ()
        }
    }

    position.0.abs() + position.1.abs()
}


pub fn result_2(input: InputType) -> i64
{   
    let mut orientation = 0;
    let mut position: (i64,i64) = (0,0);
    let mut seen: Vec<(i64,i64)> = vec![];

    'main: for (d,i) in input {
        if d == 'R' {
            orientation = (orientation+1)%4;
        } else {
            orientation = (orientation-1)%4;
            if orientation < 0 {
                orientation = 3;
            }
        }

        let movement = match orientation {
            0 => (1,0),
            1 => (0,1),
            2 => (-1,0),
            _ => (0,-1)
        };

        for _ in 0..i {
            position.0 += movement.0;
            position.1 += movement.1;
            if seen.contains(&position) {
                break 'main;
            } else {
                seen.push(position);
            }
        }
    }

    position.0.abs() + position.1.abs()
}
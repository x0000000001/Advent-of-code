use std::fs;

pub type InputType = Mine;

pub struct Cart {
    facing: u8,
    state: u8,
    x: usize,
    y: usize,
}

pub struct Mine {
    carts: Vec<Cart>,
    map: Vec<Vec<char>>,
}

fn iter_tick(g: &mut Mine) -> Option<(usize, usize)> {
    g.carts.sort_by_key(|c| c.x * g.map.len() + c.y);

    for cart in g.carts.iter_mut() {
        (cart.x, cart.y) = match cart.facing {
            0 => (cart.x - 1, cart.y),
            1 => (cart.x, cart.y + 1),
            2 => (cart.x + 1, cart.y),
            3 => (cart.x, cart.y - 1),
            _ => panic!(),
        };

        match g.map[cart.x][cart.y] {
            '-' | '|' => (),
            '+' => {
                match cart.state {
                    0 => cart.facing = (cart.facing + 3) % 4,
                    2 => cart.facing = (cart.facing + 1) % 4,
                    _ => panic!(),
                }
                cart.state = (cart.state + 1) % 3
            }
            '/' => {
                cart.facing = match cart.facing {
                    0 => 1,
                    3 => 2,
                    2 => 3,
                    1 => 0,
                    _ => panic!(),
                }
            }
            '\\' => {
                cart.facing = match cart.facing {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    // collision check
    for i in 0..g.carts.len() {
        for j in 0..g.carts.len() {
            if g.carts[i].x == g.carts[j].x && g.carts[i].y == g.carts[j].y {
                return Some((g.carts[i].x, g.carts[i].y));
            }
        }
    }

    None
}

pub fn result_1(mut input: InputType) -> i64 {
    print_map(&input);
    loop {
        if let Some(x) = iter_tick(&mut input) {
            println!("{:?}", x);
            break;
        }

        print_map(&input);
    }

    0
}

fn print_map(input: &Mine) {
    for i in 0..input.map.len() {
        let mut s = "".to_string();
        for j in 0..input.map[0].len() {
            for cart in input.carts.iter() {
                if cart.x == i && cart.y == j {
                    s += match cart.facing {
                        0 => "^",
                        1 => ">",
                        2 => "v",
                        3 => "<",
                        _ => panic!(),
                    };
                } else {
                    s += &input.map[i][j].to_string();
                }
            }
        }

        println!("{s}");
    }
}

pub fn result_2(input: InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut input: Vec<Vec<char>> = contents
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut carts = vec![];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let facing = match input[i][j] {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => 7,
            };

            if facing == 7 {
                continue;
            }

            carts.push(Cart {
                facing,
                state: 0,
                x: i,
                y: j,
            });

            input[i][j] = match facing {
                0 | 2 => '-',
                1 | 3 => '|',
                _ => panic!(),
            };
        }
    }

    Mine {
        carts: carts,
        map: input,
    }
}

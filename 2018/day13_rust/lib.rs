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

fn iter_tick(g: &mut Mine) -> Option<usize> {
    g.carts.sort_by_key(|c| c.x * g.map.len() + c.y);

    for cart in g.carts.iter_mut() {
        (cart.x, cart.y) = match cart.facing {
            0 => (cart.x - 1, cart.y),
            1 => (cart.x, cart.y + 1),
            2 => (cart.x + 1, cart.y),
            3 => (cart.x, cart.y - 1),
            _ => panic!(),
        };

        // collision check

        for i in 0..g.carts.len() {
            if g.carts[i].x == cart.x && g.carts[i].y == y {
                return;
            }
        }

        match g.map[cart.x][cart.y] {
            '-' | '|' => (),
        }
    }

    None
}

pub fn result_1(mut input: InputType) -> i64 {
    loop {
        if let Some(x) = iter_tick(&mut input) {
            return x as i64;
        }
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
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut carts = vec![];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let facing = match input[i][j] {
                '^' => 0,
                '>' => 1,
                'v' => 3,
                '<' => 4,
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

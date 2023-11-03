use crate::Solution;

type InputType = Mine;

pub struct Cart {
    facing: u8,
    state: u8,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridState {
    Free,
    Occupied,
    Collision,
}

pub struct Mine {
    carts: Vec<Cart>,
    map: Vec<Vec<char>>,
    carts_map: Vec<Vec<GridState>>,
}

// returns vec of crashes coordinates
fn iter_tick(g: &mut Mine) -> Vec<(usize, usize)> {
    g.carts.sort_by_key(|c| c.x * g.map.len() + c.y);
    let mut collisions = vec![];

    for cart in g.carts.iter_mut() {
        if g.carts_map[cart.x][cart.y] == GridState::Collision {
            continue;
        } else {
            g.carts_map[cart.x][cart.y] = GridState::Free;
        }

        (cart.x, cart.y) = match cart.facing {
            0 => (cart.x - 1, cart.y),
            1 => (cart.x, cart.y + 1),
            2 => (cart.x + 1, cart.y),
            3 => (cart.x, cart.y - 1),
            _ => panic!(),
        };

        if g.carts_map[cart.x][cart.y] == GridState::Occupied {
            g.carts_map[cart.x][cart.y] = GridState::Collision;
            collisions.push((cart.x, cart.y));
            continue;
        } else {
            g.carts_map[cart.x][cart.y] = GridState::Occupied;
        }

        match g.map[cart.x][cart.y] {
            '-' | '|' => (),
            '+' => {
                match cart.state {
                    0 => cart.facing = (cart.facing + 3) % 4,
                    2 => cart.facing = (cart.facing + 1) % 4,
                    1 => (),
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

    collisions
}

#[allow(dead_code)]
fn print_map(input: &Mine) {
    for i in 0..input.map.len() {
        let mut s = "".to_string();
        for j in 0..input.map[0].len() {
            let mut was_a_cart = false;
            for cart in input.carts.iter() {
                if cart.x == i && cart.y == j {
                    s += match cart.facing {
                        0 => "^",
                        1 => ">",
                        2 => "v",
                        3 => "<",
                        _ => panic!(),
                    };
                    was_a_cart = true;
                    break;
                }
            }
            if !was_a_cart {
                s += &input.map[i][j].to_string();
            }
        }

        println!("{s}");
    }
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    // print_map(&input);
    loop {
        if let Some(x) = iter_tick(&mut input).pop() {
            return Solution::from(format!("{:?}", x));
        }

        // print_map(&input);
    }
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    // print_map(&input);
    while input.carts.len() > 1 {
        let collisions = iter_tick(&mut input);
        input.carts = input
            .carts
            .into_iter()
            .filter(|cart| {
                for &(collx, colly) in collisions.iter() {
                    if cart.x == collx && cart.y == colly {
                        return false;
                    }
                }

                true
            })
            .collect();

        collisions
            .into_iter()
            .for_each(|(x, y)| input.carts_map[x][y] = GridState::Free);

        // print_map(&input);
    }

    Solution::from(format!("{:?}", (input.carts[0].y, input.carts[0].x)))
}

fn parse(s: String) -> InputType {
    let mut map: Vec<Vec<char>> = s
        .lines()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut carts = vec![];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let facing = match map[i][j] {
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

            map[i][j] = match facing {
                0 | 2 => '|',
                1 | 3 => '-',
                _ => panic!(),
            };
        }
    }

    let mut carts_map = vec![vec![GridState::Free; map[0].len()]; map.len()];

    for c in carts.iter() {
        carts_map[c.x][c.y] = GridState::Occupied;
    }

    Mine {
        carts,
        map,
        carts_map,
    }
}

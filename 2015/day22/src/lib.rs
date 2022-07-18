use std::collections::HashMap;
// 900 too low
// 1362 too high
// hit points, damage, armor
pub type InputType = (u64,u64);

// player, boss, timers
type KeyType = ((u64,u64),(u64,u64),(u64,u64,u64));

const SPELL_COSTS: &[u64] = &[53,73,113,173,229];

fn boss_turn(p: &mut (u64,u64), b: &mut (u64,u64), t: &mut (u64,u64,u64)) {
    if t.1 > 0 {
        t.1 -= 1;
        if b.0 > 3 {
            b.0 -= 3;
        } else {
            return;
        }
    }

    if t.2 > 0 {
        t.2 -= 1;
        p.1 += 101;
    }


    if t.0 > 0 {
        t.0 -= 1;
        let val = if b.1 > 7 { b.1 -7} else {1};
        if p.0 < val {
            p.0 = 0;
        } else {
            p.0 -= val;
        }
    } else {
        if p.0 < b.1 {
            p.0 = 0;
        } else {
            p.0 -= b.1;
        }
    } 
}

// 0 => player looses (no karma or unvalid spell)
// 1 => player wins by effect
// 2 => player wins by spell
// 3 => player doesn't win this round, game can continue
fn player_turn(p: &mut (u64,u64), b: &mut (u64,u64), t: &mut (u64,u64,u64), spell: usize) -> u8 {
    // timers
    if spell == 2 && t.0 > 0 {
        return 0;
    } else if spell == 3 && t.1 > 0 {
        return 0;
    } else if spell == 4 && t.2 > 0 {
        return 0;
    }

    // effects
    if t.1 > 0 {
        if b.0 > 3 {
            t.1 -= 1;
            b.0 -= 3;
        } else {
            return 1;
        }
    }

    if t.2 > 0 {
        t.2 -= 1;
        p.1 += 101;
    }

    if t.0 > 0 {
        t.0 -= 1;
    }


    // karma
    if SPELL_COSTS[spell] > p.1 {
        return 0;
    } else {
        p.1 -= SPELL_COSTS[spell];
    }

    // spells
    match spell {
        0 => {
            if b.0 < 4 {
                b.0 = 0;
            } else {
                b.0 -= 4;
            }
        },
        1 => {
            if b.0 < 3 {
                b.0 = 0;
            } else {
                b.0 -= 2;
            };
            p.0 += 2;
        }, 
        2 => {
            t.0 = 6;
        }, 
        3 => {
            t.1 = 6;
        },
        4 => {
            t.2 = 5;
        },
        _ => ()
    }

    if b.0 <= 0 {
        2
    } else {
        3
    }
}

// boss (hitpoints, damages)
// player : (hitpoints, karma)
fn min_karma(player: (u64,u64), boss: (u64,u64), timers: (u64,u64,u64), visited: &mut HashMap<KeyType, Option<u64>>) -> Option<u64>{

    // println!("{:?} {:?} {:?}", player, boss, timers);
    let key = (player.clone(), boss.clone(), timers.clone());

    // if let Some(res) = visited.get(&(player, boss, timers)) {
    //     return *res;
    // } else {
    //     visited.insert(key.clone(), None);
    // }

    let mut min = None;

    for spell in 0..5 {
        let (mut p, mut b, mut t) = (player.clone(), boss.clone(), timers.clone());
        let mut candidate = None;

        match player_turn(&mut p, &mut b, &mut t, spell) {
            0 => (),
            1 => candidate = Some(0),
            2 => candidate = Some(SPELL_COSTS[spell]),
            3 => candidate =  {
                boss_turn(&mut p, &mut b, &mut t);
                if b.0 <= 0 {
                    Some(SPELL_COSTS[spell])
                } else if p.0 > 0 {
                    if let Some(res) = min_karma(p, b, t, visited) {
                        Some(res + SPELL_COSTS[spell])
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => ()
        }

        if let Some(c) = candidate {
            if min.is_none() || min.unwrap() > c {
                min = Some(c);
            }
        }
    }

    visited.insert(key, min);


    min
}

pub fn result_1(boss: InputType) -> i64
{
    min_karma((50,500), boss, (0,0,0), &mut HashMap::new()).unwrap_or(0) as i64
}


pub fn result_2(_input: InputType) -> i64
{   
    0
}
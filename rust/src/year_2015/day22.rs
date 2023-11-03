use std::collections::HashMap;

use crate::Solution;

// hit points, damage
pub type InputType = (u64, u64);

// player, boss, timers
type KeyType = ((u64, u64), (u64, u64), (u64, u64, u64));

const SPELL_COSTS: &[u64] = &[53, 73, 113, 173, 229];

enum GameResult {
    // stores mana used
    Continue(u64),
    PlayerWon(u64),
    BossWon,
}

fn boss_turn(p: &mut (u64, u64), b: &mut (u64, u64), t: &mut (u64, u64, u64)) -> GameResult {
    if t.1 > 0 {
        t.1 -= 1;
        if b.0 > 3 {
            b.0 -= 3;
        } else {
            b.0 = 0;
            return GameResult::PlayerWon(0);
        }
    }

    if t.2 > 0 {
        t.2 -= 1;
        p.1 += 101;
    }

    if t.0 > 0 {
        t.0 -= 1;
        let val = if b.1 > 7 { b.1 - 7 } else { 1 };
        if p.0 <= val {
            return GameResult::BossWon;
        } else {
            p.0 -= val;
        }
    } else {
        if p.0 <= b.1 {
            return GameResult::BossWon;
        } else {
            p.0 -= b.1;
        }
    }

    GameResult::Continue(0)
}

fn player_turn(
    p: &mut (u64, u64),
    b: &mut (u64, u64),
    t: &mut (u64, u64, u64),
    spell: usize,
    hardmode: bool,
) -> GameResult {
    if hardmode {
        if p.0 <= 1 {
            return GameResult::BossWon;
        } else {
            p.0 -= 1;
        }
    }

    // bleeding effect
    if t.1 > 0 {
        if b.0 > 3 {
            t.1 -= 1;
            b.0 -= 3;
        } else {
            b.0 = 0;
            return GameResult::PlayerWon(0);
        }
    }

    // other effects
    if t.2 > 0 {
        t.2 -= 1;
        p.1 += 101;
    }

    if t.0 > 0 {
        t.0 -= 1;
    }

    // "However, effects can be started on the same turn they end."

    // timers
    if spell == 2 && t.0 > 0 {
        return GameResult::BossWon;
    } else if spell == 3 && t.1 > 0 {
        return GameResult::BossWon;
    } else if spell == 4 && t.2 > 0 {
        return GameResult::BossWon;
    }

    // karma
    if SPELL_COSTS[spell] > p.1 {
        return GameResult::BossWon;
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
        }
        1 => {
            if b.0 < 3 {
                b.0 = 0;
            } else {
                b.0 -= 2;
            };
            p.0 += 2;
        }
        2 => {
            t.0 = 6;
        }
        3 => {
            t.1 = 6;
        }
        4 => {
            t.2 = 5;
        }
        _ => (),
    }

    if b.0 <= 0 {
        GameResult::PlayerWon(SPELL_COSTS[spell])
    } else {
        GameResult::Continue(SPELL_COSTS[spell])
    }
}

/// Djisktra
// boss (hitpoints, damages)
// player : (hitpoints, karma)
fn min_karma(player: (u64, u64), boss: (u64, u64), hardmode: bool) -> u64 {
    let base_key: KeyType = (player.clone(), boss.clone(), (0, 0, 0));
    // node, cost
    let mut to_explore: Vec<(KeyType, u64)> = vec![];
    let mut visited: HashMap<KeyType, u64> = HashMap::new();
    to_explore.push((base_key, 0));

    while !to_explore.is_empty() {
        to_explore.sort_by(|(_, c0), (_, c1)| c1.cmp(c0));
        let (node, cost) = to_explore.pop().unwrap();

        if node.1 .0 <= 0 {
            return cost;
        }

        let (p_node, b_node, t_node) = node;

        for spell in 0..5 {
            let (mut p, mut b, mut t) = (p_node.clone(), b_node.clone(), t_node.clone());
            let mut candidate: Option<(KeyType, u64)> = None;

            match player_turn(&mut p, &mut b, &mut t, spell, hardmode) {
                GameResult::BossWon => (),
                GameResult::PlayerWon(mana) => candidate = Some(((p, b, t), mana)),
                GameResult::Continue(mana) => {
                    match boss_turn(&mut p, &mut b, &mut t) {
                        // player can't use mana on boss' turn
                        GameResult::Continue(_) => candidate = Some(((p, b, t), mana)),
                        GameResult::BossWon => (),
                        GameResult::PlayerWon(_) => candidate = Some(((p, b, t), mana)),
                    }
                }
            }

            if let Some(mut nn) = candidate {
                // add karma already used to get to that node
                nn.1 += cost;
                // println!("{:?}", candidate);
                if visited.contains_key(&nn.0) {
                    continue;
                }

                let mut insert_new_node = true;

                for i in 0..to_explore.len() {
                    if to_explore[i].0 == nn.0 {
                        to_explore[i].1 = to_explore[i].1.min(nn.1);
                        insert_new_node = false;
                        break;
                    }
                }

                if insert_new_node {
                    to_explore.push(nn);
                }
            }
        }

        visited.insert(node, cost);
    }

    0
}

pub fn part1(s: String) -> Solution {
    let boss = parse(s);

    Solution::from(min_karma((50, 500), boss, false) as i64)
}

pub fn part2(s: String) -> Solution {
    let boss = parse(s);

    Solution::from(min_karma((50, 500), boss, true) as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    (
        input[0].split_whitespace().last().unwrap().parse().unwrap(),
        input[1].split_whitespace().last().unwrap().parse().unwrap(),
    )
}

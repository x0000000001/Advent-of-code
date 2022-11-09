// hit points, damage, armor
pub type InputType = (i64,i64,i64);

// cost, damage, armor
const WEAPONS: &[(i64,i64,i64)] = &[(8,4,0),(10,5,0),(25,6,0),(40,7,0),(74,8,0)];
const ARMORS: &[(i64,i64,i64)] = &[(13,0,1),(31,0,2),(53,0,3),(75,0,4),(102,0,5),(0,0,0)];
const RINGS: &[(i64,i64,i64)] = &[(25,1,0),(50,2,0),(100,3,0),(20,0,1),(40,0,2),(80,0,3),(0,0,0)];

// true if player, false if boss
fn wins(mut player: (i64,i64,i64), mut boss: (i64,i64,i64)) -> bool {
    loop {
        boss.0 -= (player.1 - boss.2).max(1);
        if boss.0 <= 0 {
            return true;
        }

        player.0 -= (boss.1 - player.2).max(1);
        if player.0 <= 0 {
            return false;
        }
    }
}

pub fn result_1(input: InputType) -> i64
{
    let mut min = i64::MAX;
    for w in WEAPONS {
        for a in ARMORS {
            for r0 in RINGS {
                for r1 in RINGS {
                    let cost = w.0 + a.0 + r0.0 + r1.0;
                    if cost < min && wins((100, w.1+a.1+r0.1+r1.1, w.2+a.2+r0.2+r1.2), input.clone()) {
                        min = cost;
                    } 
                }
            }
        }
    }
    min
}


pub fn result_2(input: InputType) -> i64
{   
    let mut max = 0;
    for w in WEAPONS {
        for a in ARMORS {
            for r0 in RINGS {
                for r1 in RINGS {
                    let cost = w.0 + a.0 + r0.0 + r1.0;
                    if cost > max && !wins((100, w.1+a.1+r0.1+r1.1, w.2+a.2+r0.2+r1.2), input.clone()) {
                        max = cost;
                    } 
                }
            }
        }
    }
    max
}
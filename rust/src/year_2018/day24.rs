use std::collections::HashSet;

use crate::Solution;

type InputType = (Army, Army);

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Group {
    units_count: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

pub type Army = Vec<Group>;

impl Group {
    fn effective_power(&self) -> usize {
        self.units_count * self.attack_damage
    }

    fn take_damages(&mut self, damages_count: usize) -> usize {
        if damages_count == 0 {
            return 0;
        }

        let dead_units = (damages_count / self.hit_points).min(self.units_count);
        self.units_count -= dead_units;
        dead_units
    }

    fn is_alive(&self) -> bool {
        self.units_count > 0
    }

    fn choose_ennemy_group<'a>(
        &self,
        groups: &Vec<&'a Group>,
        available_groups: &HashSet<usize>,
    ) -> Option<usize> {
        // for (_, g) in groups
        //     .iter()
        //     .enumerate()
        //     .filter(|(i, _)| available_groups.contains(i))
        // {
        //     println!(
        //         "group unknown would deal {} damages to group unknown",
        //         potential_damages_dealt(self, g)
        //     );
        // }

        // println!();

        let (chosen_group_i, chosen_group) = groups
            .into_iter()
            .enumerate()
            .filter(|(i, _)| available_groups.contains(i))
            .max_by(|(_, g0), (_, g1)| {
                match potential_damages_dealt(self, g0).cmp(&potential_damages_dealt(self, g1)) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {
                        match g0.effective_power().cmp(&g1.effective_power()) {
                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => g0.initiative.cmp(&g1.initiative),
                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        }
                    }
                }
            })
            .unwrap();

        if potential_damages_dealt(self, chosen_group) == 0 {
            None
        } else {
            Some(chosen_group_i)
        }
    }
}

fn potential_damages_dealt(attacking_group: &Group, defending_group: &Group) -> usize {
    if defending_group
        .immunities
        .contains(&attacking_group.attack_type)
    {
        0
    } else if defending_group
        .weaknesses
        .contains(&attacking_group.attack_type)
    {
        attacking_group.effective_power() * 2
    } else {
        attacking_group.effective_power()
    }
}

fn choose_targets<'a>(attacking_army: &Army, defending_army: &Army) -> Vec<Option<usize>> {
    let mut ordered_groups: Vec<(usize, &Group)> = attacking_army.iter().enumerate().collect();
    ordered_groups.sort_by(|(_, g0), (_, g1)| {
        match g1.effective_power().cmp(&g0.effective_power()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => g1.initiative.cmp(&g0.initiative),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    });

    let all_choices = Vec::from_iter(defending_army.iter());
    let mut available_choices =
        HashSet::from_iter((0..defending_army.len()).filter(|i| defending_army[*i].is_alive()));
    let mut choices_made = vec![None; defending_army.len()];

    for (group_index, group) in ordered_groups {
        if available_choices.len() == 0 {
            break;
        }

        if let Some(group_choice_i) = group.choose_ennemy_group(&all_choices, &available_choices) {
            choices_made[group_index] = Some(group_choice_i);
            available_choices.remove(&group_choice_i);
        }
    }

    choices_made
}

/// Returns units dead during this round.
fn one_round(army0: &mut Army, army1: &mut Army) -> usize {
    // println!("Immune system :");
    // for (i, g) in army0.iter().enumerate() {
    //     if g.units_count == 0 {
    //         continue;
    //     }
    //     println!("Group {} contains {} units", i + 1, g.units_count);
    // }

    // println!("Infection :");
    // for (i, g) in army1.iter().enumerate() {
    //     if g.units_count == 0 {
    //         continue;
    //     }
    //     println!("Group {} contains {} units", i + 1, g.units_count);
    // }

    let army0_choices = choose_targets(army0, army1);
    let army1_choices = choose_targets(army1, army0);
    let mut dead_units = 0;

    // army index, index in army
    let mut all_groups: Vec<(&Group, usize, usize)> = army0
        .iter()
        .enumerate()
        .map(|(i, g)| (g, 0, i))
        .chain(army1.iter().enumerate().map(|(i, g)| (g, 1, i)))
        .collect();
    all_groups.sort_by_key(|(g, _, _)| -(g.initiative as i64));
    let attack_order: Vec<(usize, usize)> =
        all_groups.into_iter().map(|(_, ai, i)| (ai, i)).collect();

    for (army_index, index_in_army) in attack_order {
        if army_index == 0 {
            if let Some(target_choix_i) = army0_choices[index_in_army] {
                let attacking_group: &mut Group = &mut army0[index_in_army];
                let defending_group = &mut army1[target_choix_i];
                let damages = potential_damages_dealt(&attacking_group, &defending_group);

                if !attacking_group.is_alive() || !defending_group.is_alive() || damages == 0 {
                    continue;
                }

                let t = defending_group.take_damages(damages);
                dead_units += t;

                // println!(
                //     "Immune system group {} attacks defending group {}, killing {} units",
                //     index_in_army + 1,
                //     target_choix_i + 1,
                //     t
                // );
            }
        } else {
            if let Some(target_choix_i) = army1_choices[index_in_army] {
                let attacking_group: &mut Group = &mut army1[index_in_army];
                let defending_group = &mut army0[target_choix_i];
                let damages = potential_damages_dealt(&attacking_group, &defending_group);

                if !attacking_group.is_alive() || !defending_group.is_alive() || damages == 0 {
                    continue;
                }

                let t = defending_group.take_damages(damages);
                dead_units += t;

                // println!(
                //     "Infection group {} attacks defending group {}, killing {} units",
                //     index_in_army + 1,
                //     target_choix_i + 1,
                //     t
                // );
            }
        }
    }

    // println!();

    dead_units
}

fn army_alive_units(army: &Army) -> usize {
    army.iter().map(|g| g.units_count).sum()
}

fn army_is_dead(army: &Army) -> bool {
    army_alive_units(army) == 0
}

// units left if winning
fn is_army_0_winning(mut army0: Army, mut army1: Army) -> Option<usize> {
    let mut rounds_with_0_deaths = 0;

    while !army_is_dead(&army0) && !army_is_dead(&army1) {
        if one_round(&mut army0, &mut army1) == 0 {
            rounds_with_0_deaths += 1;
        } else {
            rounds_with_0_deaths = 0;
        }

        if rounds_with_0_deaths > 10 {
            break;
        }
    }

    let army_0_is_dead = army_is_dead(&army0);
    let army_1_is_dead = army_is_dead(&army1);

    if !army_0_is_dead && !army_1_is_dead {
        return None;
    } else if army_0_is_dead {
        None
    } else {
        Some(army_alive_units(&army0))
    }
}

fn create_boosted_group(army: &Army, boost: usize) -> Army {
    let mut boosted_army = army.clone();
    for i in 0..army.len() {
        boosted_army[i].attack_damage += boost;
    }
    boosted_army
}

pub fn part1(s: String) -> Solution {
    let (mut army0, mut army1) = parse(s);

    while !army_is_dead(&army0) && !army_is_dead(&army1) {
        one_round(&mut army0, &mut army1);
    }

    Solution::from(
        (if army_is_dead(&army0) {
            army_alive_units(&army1)
        } else {
            army_alive_units(&army0)
        }) as i64,
    )
}

pub fn part2(s: String) -> Solution {
    let (army0, army1) = parse(s);

    let mut min_boost = 0;
    let mut max_boost = 100000;
    let mut alive;

    if let Some(left_units) =
        is_army_0_winning(create_boosted_group(&army0, max_boost), army1.clone())
    {
        alive = left_units;
    } else {
        return Solution::NotFound;
    }

    while min_boost != max_boost - 1 {
        let mid_boost = (max_boost + min_boost) / 2;
        match is_army_0_winning(create_boosted_group(&army0, mid_boost), army1.clone()) {
            Some(units_left) => {
                alive = units_left;
                max_boost = mid_boost;
            }
            None => min_boost = mid_boost,
        }
    }

    Solution::from(alive as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let group_from_line = |mut l: String| -> Group {
        let mut g = Group {
            units_count: 0,
            hit_points: 0,
            attack_damage: 0,
            attack_type: "".to_string(),
            initiative: 0,
            weaknesses: vec![],
            immunities: vec![],
        };

        if let Some(beggin_index) = l.find("(") {
            let end_index = l.find(")").unwrap();
            let mut special_part = l
                .drain((beggin_index - 1)..(end_index + 1))
                .collect::<String>();
            special_part = special_part.replace(" (", "");
            special_part = special_part.replace(")", "");

            let parts = special_part
                .split("; ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            for part in parts {
                let part = part.replace(",", "");
                let part_words = part.split(" ").collect::<Vec<&str>>();
                let attributes = part_words[2..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                match part_words[0] {
                    "weak" => g.weaknesses = attributes,
                    "immune" => g.immunities = attributes,
                    _ => panic!(),
                }
            }
        }

        let words = l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

        g.units_count = words[0].parse().unwrap();
        g.hit_points = words[4].parse().unwrap();
        g.attack_damage = words[12].parse().unwrap();
        g.attack_type = words[13].clone();
        g.initiative = words[17].parse().unwrap();

        g
    };

    let mut army0: Army = vec![];
    let mut army1: Army = vec![];
    let mut current_army = &mut army0;

    for l in input {
        if l == "Immune System:" {
            continue;
        }

        if l == "Infection:" {
            current_army = &mut army1;
            continue;
        }

        current_army.push(group_from_line(l));
    }

    (army0, army1)
}

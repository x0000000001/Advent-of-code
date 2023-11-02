use std::{collections::HashMap, fs};

// (result, (quantity, materials))
pub type InputType = HashMap<String, (usize, Vec<(usize, String)>)>;

fn ore_for_n_fuel(input: &InputType, n: usize) -> usize {
    let mut required_materials: HashMap<&str, usize> = HashMap::new();
    let mut surproduced_quantities: HashMap<&str, usize> = HashMap::new();
    required_materials.insert("FUEL", n);

    while required_materials.len() > 1 || required_materials.get("ORE").is_none() {
        let mut new_mats: HashMap<&str, usize> = HashMap::new();

        for (product, required) in required_materials.iter() {
            if let Some((produced_per_reaction, ingredients)) = input.get(*product) {
                let surplus = surproduced_quantities.entry(product.clone()).or_insert(0);
                let taken_from_already_produced = (*surplus).min(*required);
                *surplus -= taken_from_already_produced;
                let needed = required - taken_from_already_produced;
                let reactions_count = (needed / produced_per_reaction)
                    + if needed % produced_per_reaction == 0 {
                        0
                    } else {
                        1
                    };

                *surplus += reactions_count * produced_per_reaction - needed;

                for (quantity, ing) in ingredients {
                    let entry = new_mats.entry(ing).or_insert(0);
                    *entry += quantity * reactions_count;
                }
            } else {
                // "ORE" case
                let entry = new_mats.entry(&product).or_insert(0);
                *entry += required;
            }
        }

        required_materials = new_mats;
    }

    *required_materials.get("ORE").unwrap()
}

pub fn result_1(input: InputType) -> i64 {
    ore_for_n_fuel(&input, 1) as i64
}

// dichotomy
pub fn result_2(input: InputType) -> i64 {
    let available: usize = 1000000000000;
    let mut a: usize = 1000000000000;
    let mut b: usize = 0;

    while (a - b) > 1 {
        let m = (a + b) / 2;
        let ore_needed = ore_for_n_fuel(&input, m);
        if ore_needed > available {
            a = m;
        } else {
            b = m;
        }
    }

    b as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let get_quant = |w: &str| -> (usize, String) {
        let mut temp = w.split_whitespace();
        (
            temp.next().unwrap().parse().unwrap(),
            temp.next().unwrap().to_string(),
        )
    };

    let mut res = HashMap::new();

    input.into_iter().for_each(|l| {
        let temp = l.split(" => ").collect::<Vec<&str>>();

        let ingredients = temp[0].split(", ").map(|w| get_quant(w)).collect();

        let result = get_quant(temp[1]);

        res.insert(result.1, (result.0, ingredients));
    });

    res
}

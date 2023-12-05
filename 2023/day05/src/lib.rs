mod helpers;

pub use helpers::Solution;

#[derive(Default)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<(usize, usize, usize)>,
}

type InputType = (Vec<usize>, Vec<Mapping>);

fn find_dest(from_val: usize, mapping: &Mapping) -> usize {
    for &(dest, source, range) in mapping.ranges.iter() {
        if from_val >= source && from_val < (source + range) {
            return dest + (from_val - source);
        }
    }

    from_val
}

fn find_location(mut seed: usize, mappings: &Vec<Mapping>) -> usize {
    for mapping in mappings.iter() {
        seed = find_dest(seed, mapping);
    }

    seed
}

pub fn part1(s: String) -> Solution {
    let (seeds, mappings) = parse(s);

    Solution::from(
        seeds
            .iter()
            .map(|&s| find_location(s, &mappings))
            .min()
            .unwrap() as u64,
    )
}

fn find_dest_ranges((start, end): (usize, usize), mapping: &Mapping) -> Vec<(usize, usize)> {
    let mut dest_ranges = vec![];
    let mut x = start;

    for &(dest_start, source_start, range) in mapping.ranges.iter() {
        let source_end = source_start + range - 1;

        if x > end {
            break;
        }

        if source_end < x {
            continue;
        }

        if x < source_start {
            dest_ranges.push((x, (source_start - 1).min(end)));
        }

        if source_start <= end {
            dest_ranges.push((
                source_start.max(start) + dest_start - source_start,
                source_end.min(end) + dest_start - source_start,
            ))
        }

        x = source_end + 1;
    }

    if x < end {
        dest_ranges.push((x, end));
    }

    dest_ranges
}

fn find_location_ranges(
    seed_range: (usize, usize),
    mappings: &Vec<Mapping>,
) -> Vec<(usize, usize)> {
    let mut ranges = vec![seed_range];

    for mapping in mappings.iter() {
        ranges = ranges
            .into_iter()
            .map(|r| find_dest_ranges(r, mapping))
            .flatten()
            .collect();
    }

    ranges
}

pub fn part2(s: String) -> Solution {
    let (seeds, mut mappings) = parse(s);

    for i in 0..mappings.len() {
        mappings[i].ranges.sort_by_key(|&(_, source, _)| source);
    }

    let seeds_pairs = (0..(seeds.len() / 2))
        .map(|i| (seeds[i * 2], seeds[i * 2 + 1]))
        .collect::<Vec<(usize, usize)>>();

    Solution::from(
        seeds_pairs
            .into_iter()
            .map(|(start, length)| find_location_ranges((start, start + length - 1), &mappings))
            .flatten()
            .map(|(start, _)| start)
            .min()
            .unwrap() as u64,
    )
}

fn parse_group(s: &str) -> Mapping {
    let mut mapping = Mapping::default();

    for (i, l) in s.lines().enumerate() {
        if i == 0 {
            let w = l.split_whitespace().collect::<Vec<&str>>();
            let w1 = w[0].split("-to-").collect::<Vec<&str>>();

            mapping.from = w1[0].to_string();
            mapping.to = w1[1].to_string();

            continue;
        }

        let w = l
            .split_whitespace()
            .map(|ws| ws.parse().unwrap())
            .collect::<Vec<usize>>();

        mapping.ranges.push((w[0], w[1], w[2]))
    }

    mapping
}

fn parse(s: String) -> InputType {
    let mut groups = s.split_terminator("\n\n").collect::<Vec<&str>>();

    let seeds_s = groups.remove(0);
    let mut words = seeds_s.split_whitespace();
    words.next();

    (
        words.map(|w| w.parse().unwrap()).collect(),
        groups.iter().map(|g| parse_group(g)).collect(),
    )
}

use std::{collections::HashMap, fs::read_to_string, io::Error};

use crate::utils::get_input_file_name;

// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4

fn parse_seeds(line: &str) -> Vec<i64> {
    line.split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|num| num.to_string().trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_mapping(chunk: &str) -> (&str, &str, Vec<Entry>) {
    let source_to_dist = chunk
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .split('-')
        .collect::<Vec<&str>>();
    let source = source_to_dist[0];
    let dist = source_to_dist[2];

    let entries = chunk
        .lines()
        .skip(1)
        .map(|line| {
            let nums = line
                .split(' ')
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Entry {
                source: nums[1],
                dist: nums[0],
                range: nums[2],
            }
        })
        .collect::<Vec<Entry>>();
    (source, dist, entries)
}

fn traverse_mappings(
    starting_src: &str,
    starting_value: i64,
    mappings: &HashMap<(&str, &str), Vec<Entry>>,
) -> i64 {
    let mut result = starting_value;
    let mut current_src = starting_src;
    while let Some(((_, dist), entries)) = mappings.iter().find(|((src, _), _)| src == &current_src)
    {
        if let Some(value) = entries
            .iter()
            .find_map(|entry| entry.get_dist_value(result))
        {
            result = value;
        }
        current_src = dist;
    }
    result
}

fn reverse_traverse_mappings(
    starting_dist: &str,
    starting_value: i64,
    mappings: &HashMap<(&str, &str), Vec<Entry>>,
) -> i64 {
    let mut result = starting_value;
    let mut current_dist = starting_dist;
    while let Some(((src, _), entries)) =
        mappings.iter().find(|((_, dist), _)| dist == &current_dist)
    {
        if let Some(value) = entries.iter().find_map(|entry| entry.get_src_value(result)) {
            result = value;
        }
        current_dist = src;
    }
    result
}

fn solve_part_1(seeds: &[i64], mappings: &HashMap<(&str, &str), Vec<Entry>>) -> i64 {
    let mut location: i64 = i64::MAX;
    for seed in seeds.iter() {
        location = location.min(traverse_mappings("seed", *seed, mappings));
    }
    location
}

fn solve_part_2(seeds: &[i64], mappings: &HashMap<(&str, &str), Vec<Entry>>) -> Option<i64> {
    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    for location in 1..=999_000_000 {
        let seed = reverse_traverse_mappings("location", location, mappings);
        for range in seed_ranges.iter() {
            if range.contains(&seed) {
                return Some(location);
            }
        }
    }
    None
}

pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;
    let mut mappings: HashMap<(&str, &str), Vec<Entry>> = HashMap::new();

    let seeds: Vec<i64> = contents
        .lines()
        .next()
        .map(parse_seeds)
        .expect("seeds not on first line");

    for chunk in contents.split("\n\n").skip(1) {
        let (src, dist, entries) = parse_mapping(chunk);
        mappings.insert((src, dist), entries);
    }

    let part_1_answer = solve_part_1(&seeds, &mappings);
    println!(
        "module: {}, part 1, result: {}",
        module_path!(),
        part_1_answer
    );

    let part_2_answer = solve_part_2(&seeds, &mappings).expect("no answer found for day 5 part 2");
    println!(
        "module: {}, part 2, result: {:?}",
        module_path!(),
        part_2_answer
    );

    Ok(())
}

#[derive(Debug)]
struct Entry {
    source: i64,
    dist: i64,
    range: i64,
}

impl Entry {
    fn get_dist_value(&self, src_value: i64) -> Option<i64> {
        if src_value >= self.source && src_value <= self.source + self.range {
            let diff = src_value - self.source;
            return Some(self.dist + diff);
        }
        None
    }
    fn get_src_value(&self, dist_value: i64) -> Option<i64> {
        let diff = dist_value - self.dist;
        let src = self.source + diff;
        if src >= self.source && src <= self.source + self.range {
            return Some(src);
        }
        None
    }
}

#[test]
fn test_reverse_traverse_mappings() {
    let mappings: HashMap<(&str, &str), Vec<Entry>> = HashMap::from_iter(
        vec![
            (
                ("seed", "soil"),
                vec![
                    Entry {
                        source: 98,
                        dist: 50,
                        range: 2,
                    },
                    Entry {
                        source: 50,
                        dist: 52,
                        range: 48,
                    },
                ],
            ),
            (
                ("soil", "fertilizer"),
                vec![
                    Entry {
                        source: 15,
                        dist: 0,
                        range: 37,
                    },
                    Entry {
                        source: 52,
                        dist: 37,
                        range: 2,
                    },
                    Entry {
                        source: 0,
                        dist: 39,
                        range: 15,
                    },
                ],
            ),
            (
                ("fertilizer", "water"),
                vec![
                    Entry {
                        source: 53,
                        dist: 49,
                        range: 8,
                    },
                    Entry {
                        source: 11,
                        dist: 0,
                        range: 42,
                    },
                    Entry {
                        source: 0,
                        dist: 42,
                        range: 7,
                    },
                    Entry {
                        source: 7,
                        dist: 57,
                        range: 4,
                    },
                ],
            ),
            (
                ("water", "light"),
                vec![
                    Entry {
                        source: 18,
                        dist: 88,
                        range: 7,
                    },
                    Entry {
                        source: 25,
                        dist: 18,
                        range: 70,
                    },
                ],
            ),
            (
                ("light", "temperature"),
                vec![
                    Entry {
                        source: 77,
                        dist: 45,
                        range: 23,
                    },
                    Entry {
                        source: 45,
                        dist: 81,
                        range: 19,
                    },
                    Entry {
                        source: 64,
                        dist: 68,
                        range: 13,
                    },
                ],
            ),
            (
                ("temperature", "humidity"),
                vec![
                    Entry {
                        source: 69,
                        dist: 0,
                        range: 1,
                    },
                    Entry {
                        source: 0,
                        dist: 1,
                        range: 69,
                    },
                ],
            ),
            (
                ("humidity", "location"),
                vec![
                    Entry {
                        source: 56,
                        dist: 60,
                        range: 37,
                    },
                    Entry {
                        source: 93,
                        dist: 56,
                        range: 4,
                    },
                ],
            ),
        ]
        .into_iter(),
    );
    assert_eq!(traverse_mappings("seed", 79, &mappings), 82);
    assert_eq!(traverse_mappings("seed", 14, &mappings), 43);
    assert_eq!(traverse_mappings("seed", 55, &mappings), 86);
    assert_eq!(traverse_mappings("seed", 13, &mappings), 35);

    assert_eq!(reverse_traverse_mappings("location", 82, &mappings), 79);
    assert_eq!(reverse_traverse_mappings("location", 43, &mappings), 14);
    assert_eq!(reverse_traverse_mappings("location", 46, &mappings), 82);
}

#[test]
fn test_entry() {
    let entry = Entry {
        source: 98,
        dist: 50,
        range: 2,
    };
    assert_eq!(entry.get_dist_value(98), Some(50));
    assert_eq!(entry.get_dist_value(99), Some(51));
    assert_eq!(entry.get_dist_value(100), Some(52));
    assert_eq!(entry.get_dist_value(101), None);

    assert_eq!(entry.get_src_value(50), Some(98));
    assert_eq!(entry.get_src_value(51), Some(99));
    assert_eq!(entry.get_src_value(52), Some(100));
    assert_eq!(entry.get_src_value(53), None);
}

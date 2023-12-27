use crate::utils::get_input_file_name;
use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("unknown instruction"),
        }
    }
}

fn solve_part_1(
    instructions: &[Instruction],
    network: &HashMap<String, (String, String)>,
) -> usize {
    let mut current_node = "AAA";
    let mut steps = 0;
    for instruction in instructions.iter().cycle() {
        if let Some((l, r)) = network.get(current_node) {
            steps += 1;
            current_node = match instruction {
                Instruction::Left => l,
                Instruction::Right => r,
            };
            if current_node == "ZZZ" {
                break;
            }
        }
    }
    steps
}

fn solve_part_2(instructions: &[Instruction], network: &HashMap<String, (String, String)>) -> i64 {
    network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|node| {
            let mut seen: HashMap<&String, i64> = HashMap::new();
            let mut current_node = node;
            let mut steps = 0;
            for instruction in instructions.iter().cycle() {
                let (l, r) = network.get(current_node).unwrap();
                steps += 1;
                current_node = match instruction {
                    Instruction::Left => l,
                    Instruction::Right => r,
                };
                if current_node.ends_with('Z') {
                    // make sure this is indeed the last end node in the chain by going through the
                    // same chain again.
                    if let Some(v) = seen.get(current_node) {
                        if *v == steps {
                            break;
                        }
                    } else {
                        seen.insert(current_node, steps);
                        steps = 0;
                    }
                }
            }
            steps
        })
        .fold(1, lcm)
}

pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;

    let (instructions_str, network_str) = contents.split_once("\n\n").unwrap();

    let instructions: Vec<Instruction> = instructions_str.chars().map(|c| c.into()).collect();

    let node_re =
        Regex::new(r"(?P<start>[A-Z]{3})\s=\s\((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})\)").unwrap();
    let network: HashMap<String, (String, String)> = network_str
        .lines()
        .map(|s| {
            let caps = node_re.captures(s).unwrap();
            (
                caps["start"].to_string(),
                (caps["left"].to_string(), caps["right"].to_string()),
            )
        })
        .collect();

    let part_1_solution = solve_part_1(&instructions, &network);
    println!(
        "module: {}, part 1, result: {}",
        module_path!(),
        part_1_solution
    );

    let part_2_solution = solve_part_2(&instructions, &network);
    println!(
        "module: {}, part 1, result: {}",
        module_path!(),
        part_2_solution
    );
    Ok(())
}

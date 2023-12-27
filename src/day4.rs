use std::{collections::HashSet, fs::read_to_string};

use crate::utils::get_input_file_name;

fn card_matches(card: &str) -> usize {
    let numbers_str = card.split(':').collect::<Vec<&str>>()[1].trim();
    let numbers_parts = numbers_str.split('|').collect::<Vec<&str>>();
    let my_numbers = numbers_parts[1]
        .trim()
        .split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<HashSet<i64>>();

    numbers_parts[0]
        .trim()
        .split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .filter(|n| my_numbers.contains(n))
        .collect::<HashSet<i64>>()
        .len()
}

pub fn solve() {
    let input_file_name = get_input_file_name(module_path!());
    let mut cards: Vec<Card> = read_to_string(format!("src/{}", input_file_name))
        .unwrap()
        .lines()
        .map(|line| {
            let matches = card_matches(line);
            let points = (0..matches).fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 });
            Card {
                num: 1,
                points,
                matches,
            }
        })
        .collect::<Vec<Card>>();

    let points: i64 = cards.iter().map(|c| c.points).sum();
    println!("module: {}, part 1, result: {}", module_path!(), points);

    for i in 0..cards.len() {
        for j in i + 1..=i + cards[i].matches {
            cards[j].num += cards[i].num;
        }
    }
    let total_cards: i64 = cards.iter().map(|c| c.num).sum();
    println!(
        "module: {}, part 2, result: {}",
        module_path!(),
        total_cards
    );
}

#[derive(Debug)]
struct Card {
    num: i64,
    points: i64,
    matches: usize,
}

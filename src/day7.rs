use crate::utils::get_input_file_name;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    // all cards' labels are distinct
    HighCard,
    OnePair,
    // where two cards share one label, two other cards share a second label, and the remaining card has a third label
    TwoPair,
    // where three cards have the same label, and the remaining two cards are each different
    ThreeOfKind,
    // three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn from_cards(cards: &Vec<Card>, has_wild_card: bool) -> Self {
        let mut group = HashMap::new();
        for c in cards {
            group.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        if has_wild_card {
            if let Some(num_j) = group.clone().get(&Card::Joker) {
                if *num_j < 5 {
                    group.retain(|k, _| **k != Card::Joker);
                    let max_card = group.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
                    group.entry(*max_card).and_modify(|v| *v += num_j);
                }
            }
        }
        match group.len() {
            1 => HandType::FiveOfKind,
            2 => {
                if group.values().any(|v| *v == 1) {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if group.values().any(|v| *v == 3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("malformed hand"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}
impl Card {
    fn from_char(c: char, has_wild_card: bool) -> Card {
        match c {
            'J' => {
                if has_wild_card {
                    Card::Joker
                } else {
                    Card::J
                }
            }
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("unknown card type"),
        }
    }
}

// A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
// The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: i64,
}

impl Hand {
    fn new(cards_str: String, bid: i64, has_wild_card: bool) -> Self {
        let cards: Vec<Card> = cards_str
            .chars()
            .map(|c| Card::from_char(c, has_wild_card))
            .collect::<Vec<Card>>();

        Hand {
            cards: cards.clone(),
            bid,
            hand_type: HandType::from_cards(&cards, has_wild_card),
        }
    }
}

pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;

    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<i64>().unwrap();
            Hand::new(cards.to_string(), bid, false)
        })
        .collect();
    hands.sort();
    let solution = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i64 + 1)));
    println!("module: {}, part 1, result: {}", module_path!(), solution);

    let mut hands_with_wc: Vec<Hand> = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<i64>().unwrap();
            Hand::new(cards.to_string(), bid, true)
        })
        .collect();
    hands_with_wc.sort();
    let solution = hands_with_wc
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i64 + 1)));
    println!("module: {}, part 2, result: {}", module_path!(), solution);

    Ok(())
}

#[test]
fn test_new_hand() {
    assert_eq!(
        Hand::new("AAAAA".to_string(), 77, true),
        Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
            bid: 77,
            hand_type: HandType::FiveOfKind
        }
    );
    assert_eq!(
        Hand::new("JJJJJ".to_string(), 77, true),
        Hand {
            cards: vec![
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker
            ],
            bid: 77,
            hand_type: HandType::FiveOfKind
        }
    );
    assert_eq!(
        Hand::new("JJJJJ".to_string(), 77, false),
        Hand {
            cards: vec![Card::J, Card::J, Card::J, Card::J, Card::J],
            bid: 77,
            hand_type: HandType::FiveOfKind
        }
    );
    assert_eq!(
        Hand::new("T55J5".to_string(), 77, true),
        Hand {
            cards: vec![Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five],
            bid: 77,
            hand_type: HandType::FourOfKind
        }
    );
    assert_eq!(
        Hand::new("T55J5".to_string(), 77, false),
        Hand {
            cards: vec![Card::Ten, Card::Five, Card::Five, Card::J, Card::Five],
            bid: 77,
            hand_type: HandType::ThreeOfKind
        }
    );
    assert_eq!(
        Hand::new("T43J5".to_string(), 77, true),
        Hand {
            cards: vec![Card::Ten, Card::Four, Card::Three, Card::Joker, Card::Five],
            bid: 77,
            hand_type: HandType::OnePair
        }
    );
    assert_eq!(
        Hand::new("T43J5".to_string(), 77, false),
        Hand {
            cards: vec![Card::Ten, Card::Four, Card::Three, Card::J, Card::Five],
            bid: 77,
            hand_type: HandType::HighCard
        }
    );
}

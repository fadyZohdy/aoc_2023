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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}
impl From<char> for Card {
    fn from(c: char) -> Card {
        match c {
            'J' => Card::J,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
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
    fn new(cards: String, bid: i64) -> Hand {
        let chars: Vec<Card> = cards
            .chars()
            .map(|c| c.into())
            .collect::<Vec<Card>>()
            .into();

        let mut group = HashMap::new();
        for c in &chars {
            group.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        if let Some(num_j) = group.clone().get(&Card::J) {
            if *num_j < 5 {
                group.retain(|k, _| **k != Card::J);
                let max_card = group.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
                group.entry(*max_card).and_modify(|v| *v += num_j);
            }
        }
        let hand_type = match group.len() {
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
        };
        Hand {
            cards: chars,
            bid,
            hand_type,
        }
    }
}

pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;

    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| {
            let cards = line.split_once(" ").unwrap().0;
            let bid = line.split_once(" ").unwrap().1.parse::<i64>().unwrap();
            Hand::new(cards.to_string(), bid)
        })
        .collect();
    hands.sort();

    let solution = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i64 + 1)));
    println!("module: {}, part 2, result: {}", module_path!(), solution);
    Ok(())
}

#[test]
fn test_new_hand() {
    assert_eq!(
        Hand::new("AAAAA".to_string(), 77),
        Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
            bid: 77,
            hand_type: HandType::FiveOfKind
        }
    );
    assert_eq!(
        Hand::new("JJJJJ".to_string(), 77),
        Hand {
            cards: vec![Card::J, Card::J, Card::J, Card::J, Card::J],
            bid: 77,
            hand_type: HandType::FiveOfKind
        }
    );
    assert_eq!(
        Hand::new("T55J5".to_string(), 77),
        Hand {
            cards: vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five],
            bid: 77,
            hand_type: HandType::FourOfKind
        }
    );
    assert_eq!(
        Hand::new("T43J5".to_string(), 77),
        Hand {
            cards: vec![Card::T, Card::Four, Card::Three, Card::J, Card::Five],
            bid: 77,
            hand_type: HandType::OnePair
        }
    );
}

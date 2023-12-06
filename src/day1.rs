use std::fs::read_to_string;

use crate::utils::get_input_file_name;

fn check_number_word(chars: &Vec<char>, index: usize, length: usize, expected_number: i64) -> bool {
    if chars.len() < index + length {
        return false;
    }
    let word: String = chars[index..index + length].iter().collect();
    match word.as_str() {
        "one" => 1 == expected_number,
        "two" => 2 == expected_number,
        "three" => 3 == expected_number,
        "four" => 4 == expected_number,
        "five" => 5 == expected_number,
        "six" => 6 == expected_number,
        "seven" => 7 == expected_number,
        "eight" => 8 == expected_number,
        "nine" => 9 == expected_number,
        _ => false,
    }
}

fn is_number(chars: &Vec<char>, i: usize) -> Option<i64> {
    if chars[i].is_numeric() {
        return Some(chars[i].to_digit(10).unwrap() as i64);
    }
    match chars[i] {
        'o' => {
            if check_number_word(chars, i, 3, 1) {
                return Some(1);
            }
        }
        't' => {
            if check_number_word(chars, i, 3, 2) {
                return Some(2);
            } else if check_number_word(chars, i, 5, 3) {
                return Some(3);
            }
        }
        'f' => {
            if check_number_word(chars, i, 4, 4) {
                return Some(4);
            } else if check_number_word(chars, i, 4, 5) {
                return Some(5);
            }
        }
        's' => {
            if check_number_word(chars, i, 3, 6) {
                return Some(6);
            } else if check_number_word(chars, i, 5, 7) {
                return Some(7);
            }
        }
        'e' => {
            if check_number_word(chars, i, 5, 8) {
                return Some(8);
            }
        }
        'n' => {
            if check_number_word(chars, i, 4, 9) {
                return Some(9);
            }
        }
        _ => return None,
    };
    None
}

fn line_value(line: &str) -> i64 {
    let chars: Vec<char> = line.chars().collect();
    let mut i: usize = 0;
    let mut first: Option<i64> = None;
    let mut last: Option<i64> = None;
    while i < chars.len() {
        if let Some(number) = is_number(&chars, i) {
            if first.is_none() {
                first = Some(number);
            }
            last = Some(number);
        }
        i += 1;
    }
    format!("{}{}", first.unwrap(), last.unwrap())
        .parse::<i64>()
        .unwrap()
}

pub fn solve() {
    let mut res: i64 = 0;
    let input_file_name = get_input_file_name(module_path!());
    for line in read_to_string(format!("src/{}", input_file_name)).unwrap().lines() {
        let value = line_value(line);
        res += value;
    }
    println!("module: {}, result: {}", module_path!(), res);
}

#[test]
fn test_line_value() {
    let cases = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
        ("mchm6", 66),
        ("88sixtwo", 82),
        ("1oneightx", 18),
        ("dsevenninefour48kjs", 78),
    ];
    for (line, value) in cases.iter() {
        assert_eq!(line_value(line), *value as i64);
    }
}

#[test]
fn test_is_number() {
    let cases = [
        (("one1nine", 0), Some(1)),
        (("one1nine", 3), Some(1)),
        (("eightwothree", 0), Some(8)),
        (("eightwothree", 4), Some(2)),
        (("abcone2threexyo", 0), None),
        (("abcone2threexyo", 14), None),
    ];
    for (input, res) in cases.iter() {
        assert_eq!(
            is_number(&input.0.chars().collect::<Vec<char>>(), input.1),
            *res
        );
    }
}

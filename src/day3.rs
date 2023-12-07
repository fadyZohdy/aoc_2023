use std::fs::read_to_string;

use crate::utils::get_input_file_name;

fn scan_number(line: &Vec<u8>, mut i: usize) -> usize {
    while i < line.len() && line[i].is_ascii_digit() {
        i += 1;
    }
    i - 1
}

fn is_symbol(character: u8) -> bool {
    !character.is_ascii_digit() && character != b'.' && character != b'\n'
}

fn is_part_number(
    engine_schematic: &Vec<Vec<u8>>,
    line_num: usize,
    num_start: usize,
    num_end: usize,
) -> bool {
    // check left cell
    if num_start > 0 && is_symbol(engine_schematic[line_num][num_start - 1]) {
        return true;
    }
    // check right cell
    if num_end < engine_schematic[line_num].len() - 1
        && is_symbol(engine_schematic[line_num][num_end + 1])
    {
        return true;
    }
    // check above adjacent cells
    if line_num > 0 {
        let previous_line_num = line_num - 1;
        let mut i = if num_start == 0 {
            num_start
        } else {
            num_start - 1
        };
        while i <= num_end + 1 && i < engine_schematic[previous_line_num].len() {
            if is_symbol(engine_schematic[previous_line_num][i]) {
                return true;
            }
            i += 1;
        }
    }
    // check below adjacent cells
    if line_num < engine_schematic.len() - 1 {
        let next_line_num = line_num + 1;
        let mut i = if num_start == 0 {
            num_start
        } else {
            num_start - 1
        };
        while i <= num_end + 1 && i < engine_schematic[next_line_num].len() {
            if is_symbol(engine_schematic[next_line_num][i]) {
                return true;
            }
            i += 1;
        }
    }
    false
}

pub fn solve() {
    let mut parts_sum: i64 = 0;
    let input_file_name = get_input_file_name(module_path!());
    let engine_schematic = read_to_string(format!("src/{}", input_file_name))
        .unwrap()
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let mut line_num = 0;
    while line_num < engine_schematic.len() {
        let mut i = 0;
        while i < engine_schematic[line_num].len() {
            if engine_schematic[line_num][i].is_ascii_digit() {
                let j = scan_number(&engine_schematic[line_num], i);
                if is_part_number(&engine_schematic, line_num, i, j) {
                    let num = String::from_utf8(engine_schematic[line_num][i..=j].to_vec())
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    parts_sum += num;
                }
                i = j + 1;
            } else {
                i += 1;
            }
        }
        line_num += 1;
    }
    println!("module: {}, result: {}", module_path!(), parts_sum);
}

#[test]
fn test_scan_number() {
    assert_eq!(scan_number(&".+123..".to_string().into_bytes(), 2), 4);
    assert_eq!(scan_number(&"537..".to_string().into_bytes(), 0), 2);
}

#[test]
fn test_is_symbol() {
    assert_eq!(is_symbol(b'.'), false);
    assert_eq!(is_symbol(b'\n'), false);
    assert_eq!(is_symbol(b'1'), false);
    assert_eq!(is_symbol(b' '), true);
    assert_eq!(is_symbol(b'='), true);
    assert_eq!(is_symbol(b'('), true);
    assert_eq!(is_symbol(b')'), true);
    assert_eq!(is_symbol(b'['), true);
    assert_eq!(is_symbol(b']'), true);
    assert_eq!(is_symbol(b'{'), true);
    assert_eq!(is_symbol(b'}'), true);
    assert_eq!(is_symbol(b','), true);
    assert_eq!(is_symbol(b';'), true);
    assert_eq!(is_symbol(b':'), true);
    assert_eq!(is_symbol(b'!'), true);
    assert_eq!(is_symbol(b'?'), true);
    assert_eq!(is_symbol(b'@'), true);
    assert_eq!(is_symbol(b'#'), true);
    assert_eq!(is_symbol(b'$'), true);
    assert_eq!(is_symbol(b'%'), true);
    assert_eq!(is_symbol(b'^'), true);
    assert_eq!(is_symbol(b'&'), true);
    assert_eq!(is_symbol(b'*'), true);
    assert_eq!(is_symbol(b'_'), true);
    assert_eq!(is_symbol(b'-'), true);
    assert_eq!(is_symbol(b'+'), true);
    assert_eq!(is_symbol(b'='), true);
    assert_eq!(is_symbol(b'<'), true);
    assert_eq!(is_symbol(b'>'), true);
    assert_eq!(is_symbol(b'/'), true);
    assert_eq!(is_symbol(b'\\'), true);
    assert_eq!(is_symbol(b'|'), true);
    assert_eq!(is_symbol(b'~'), true);
    assert_eq!(is_symbol(b'`'), true);
    assert_eq!(is_symbol(b'\''), true);
    assert_eq!(is_symbol(b'"'), true);
    assert_eq!(is_symbol(b' '), true);
    assert_eq!(is_symbol(b'.'), false);
    assert_eq!(is_symbol(b'0'), false);
    assert_eq!(is_symbol(b'9'), false);
    assert_eq!(is_symbol(b'A'), true);
    assert_eq!(is_symbol(b'Z'), true);
    assert_eq!(is_symbol(b'a'), true);
}

#[test]
fn test_is_part_number() {
    let input = vec![
        ".467..114..".to_string().into_bytes(),
        "...*......".to_string().into_bytes(),
        "..35..633.".to_string().into_bytes(),
        ".*....#...".to_string().into_bytes(),
        "617.......".to_string().into_bytes(),
        ".....+.58.".to_string().into_bytes(),
        "..592.....".to_string().into_bytes(),
        "......755.".to_string().into_bytes(),
        "...$.*....".to_string().into_bytes(),
        ".664.598..".to_string().into_bytes(),
    ];
    assert_eq!(is_part_number(&input, 0, 1, 4), true);
    assert_eq!(is_part_number(&input, 0, 6, 9), false);
    assert_eq!(is_part_number(&input, 2, 2, 3), true);
    assert_eq!(is_part_number(&input, 2, 6, 8), true);
    assert_eq!(is_part_number(&input, 4, 0, 2), true);
    assert_eq!(is_part_number(&input, 5, 7, 8), false);
    assert_eq!(is_part_number(&input, 6, 2, 4), true);
    assert_eq!(is_part_number(&input, 7, 6, 8), true);
    assert_eq!(is_part_number(&input, 9, 1, 3), true);
    assert_eq!(is_part_number(&input, 9, 5, 7), true);

    let input = vec![
        "...#.".to_string().into_bytes(),
        "537..".to_string().into_bytes(),
        ".....".to_string().into_bytes(),
    ];
    assert_eq!(is_part_number(&input, 1, 0, 2), true);

    let input = vec![
        ".........".to_string().into_bytes(),
        "310+..603".to_string().into_bytes(),
        ".....@...".to_string().into_bytes(),
        "968......".to_string().into_bytes(),
    ];
    assert_eq!(is_part_number(&input, 1, 6, 8), true);
}

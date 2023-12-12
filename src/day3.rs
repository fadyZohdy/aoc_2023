use std::{collections::HashSet, fs::read_to_string};

use crate::utils::get_input_file_name;

fn scan_number(line: &Vec<u8>, mut i: usize) -> usize {
    while i < line.len() && line[i].is_ascii_digit() {
        i += 1;
    }
    i - 1
}

fn is_symbol(character: u8) -> bool {
    !character.is_ascii_digit() && character != b'.'
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

fn extract_part_numbers_and_gears(
    engine_schematic: &Vec<Vec<u8>>,
) -> (Vec<PartNumber>, HashSet<(i64, i64)>) {
    let mut part_numbers = Vec::<PartNumber>::new();
    let mut gears = HashSet::<(i64, i64)>::new();

    let mut row = 0;
    while row < engine_schematic.len() {
        let mut col = 0;
        while col < engine_schematic[row].len() {
            if engine_schematic[row][col].is_ascii_digit() {
                let j = scan_number(&engine_schematic[row], col);
                if is_part_number(&engine_schematic, row, col, j) {
                    let num = String::from_utf8(engine_schematic[row][col..=j].to_vec())
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    part_numbers.push(PartNumber {
                        num,
                        line_num: row,
                        num_start: col,
                        num_end: j,
                    });
                }
                col = j + 1;
            } else if engine_schematic[row][col] == b'*' {
                gears.insert((row as i64, col as i64));
                col += 1;
            } else {
                col += 1;
            }
        }
        row += 1;
    }
    (part_numbers, gears)
}

pub fn solve() {
    let input_file_name = get_input_file_name(module_path!());
    let engine_schematic = read_to_string(format!("src/{}", input_file_name))
        .unwrap()
        .lines()
        .map(|line| line.to_string().into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let (part_numbers, gears) = extract_part_numbers_and_gears(&engine_schematic);

    let parts_sum: i64 = part_numbers.iter().map(|pn| pn.num).sum();
    println!("module: {}, part 1, result: {}", module_path!(), parts_sum);

    let gear_ratios_sum: i64 = gears
        .iter()
        .filter_map(|gear| {
            let neigbours = part_numbers
                .iter()
                .filter(|pn| pn.to_owned().adjacent_cells().contains(gear))
                .collect::<Vec<&PartNumber>>();
            if neigbours.len() == 2 {
                Some(neigbours[0].num * neigbours[1].num)
            } else {
                None
            }
        })
        .sum();
    println!(
        "module: {}, part 2, result: {}",
        module_path!(),
        gear_ratios_sum
    );
}

#[derive(Copy, Clone, Debug)]
struct PartNumber {
    num: i64,
    line_num: usize,
    num_start: usize,
    num_end: usize,
}

impl PartNumber {
    fn adjacent_cells(self) -> HashSet<(i64, i64)> {
        let mut res = HashSet::new();
        res.insert((self.line_num as i64, self.num_start as i64 - 1));
        res.insert((self.line_num as i64, self.num_end as i64 + 1));
        let previuos_line_cells = (self.num_start as i64 - 1..=self.num_end as i64 + 1)
            .map(|i| (self.line_num as i64 - 1, i))
            .collect::<HashSet<(i64, i64)>>();
        res.extend(previuos_line_cells);
        let next_line_cells = (self.num_start as i64 - 1..=self.num_end as i64 + 1)
            .map(|i| (self.line_num as i64 + 1, i))
            .collect::<HashSet<(i64, i64)>>();
        res.extend(next_line_cells);
        res
    }
}

#[test]
fn test_scan_number() {
    assert_eq!(scan_number(&".+123..".to_string().into_bytes(), 2), 4);
    assert_eq!(scan_number(&"537..".to_string().into_bytes(), 0), 2);
}

#[test]
fn test_is_symbol() {
    assert_eq!(is_symbol(b'.'), false);
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

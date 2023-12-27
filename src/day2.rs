use std::fs::read_to_string;

use crate::utils::get_input_file_name;

struct Set {
    green: i64,
    blue: i64,
    red: i64,
}

struct Game {
    id: i64,
    sets: Vec<Set>,
}

const RED_CUBES: i64 = 12;
const GREEN_CUBES: i64 = 13;
const BLUE_CUBES: i64 = 14;

// Game 6: 8 blue, 1 red, 17 green; 7 blue; 10 green, 6 blue; 5 blue, 1 red, 11 green
fn parse_line(line: &str) -> Game {
    let parts = line.split(':').collect::<Vec<&str>>();
    let game_part = parts[0].split(' ').collect::<Vec<&str>>();
    let game_id = game_part[1].trim().parse::<i64>().unwrap();

    let sets = parts[1].split(';').map(|set_string| {
        let set_parts = set_string.split(',').collect::<Vec<&str>>();
        let mut set = Set {
            green: 0,
            blue: 0,
            red: 0,
        };
        for set_part in set_parts {
            let set_part_parts = set_part.trim().split(' ').collect::<Vec<&str>>();
            let number = set_part_parts[0].parse::<i64>().unwrap();
            let color = set_part_parts[1].trim();
            match color {
                "green" => set.green = number,
                "blue" => set.blue = number,
                "red" => set.red = number,
                _ => panic!("Unknown color: {}", color),
            }
        }
        set
    }).collect::<Vec<Set>>();
    Game {
        id: game_id,
        sets,
    }
}

fn is_game_valid(game: &Game) -> bool {
    game.sets.iter().all(|set| {
        set.green <= GREEN_CUBES && set.blue <= BLUE_CUBES && set.red <= RED_CUBES
    })
}

fn fewest_possible_cubes(game: &Game) -> Set {
    let mut max_red = i64::MIN;
    let mut max_green = i64::MIN;
    let mut max_blue = i64::MIN;
    for set in game.sets.iter() {
        if set.red > max_red {
            max_red = set.red;
        }
        if set.green > max_green {
            max_green = set.green;
        }
        if set.blue > max_blue {
            max_blue = set.blue;
        }
    }
    Set {
        green: max_green,
        blue: max_blue,
        red: max_red,
    }
}

pub fn solve() {
    let mut valid_games_sum = 0;
    let mut power_sum = 0;
    let input_file_name = get_input_file_name(module_path!());
    for line in read_to_string(format!("src/{}", input_file_name)).unwrap().lines() {
        let game = parse_line(line);
        if is_game_valid(&game) {
            valid_games_sum += game.id;
        }
        let fewest_possible_cubes = fewest_possible_cubes(&game);
        let power = fewest_possible_cubes.green * fewest_possible_cubes.blue * fewest_possible_cubes.red;
        power_sum += power;
    }
    println!("module: {}, Part 1 result: {}", module_path!(), valid_games_sum);
    println!("module: {}, Part 2 result: {}", module_path!(), power_sum);
}

#[test]
fn test_parse_line() {
    let line = "Game 6: 8 blue, 1 red, 17 green; 7 blue; 10 green, 6 blue; 5 blue, 1 red, 11 green";
    let game = parse_line(line);
    assert_eq!(game.id, 6);
    assert_eq!(game.sets.len(), 4);
    assert_eq!(game.sets[0].green, 17);
    assert_eq!(game.sets[0].blue, 8);
    assert_eq!(game.sets[0].red, 1);
    assert_eq!(game.sets[1].green, 0);
    assert_eq!(game.sets[1].blue, 7);
    assert_eq!(game.sets[1].red, 0);
    assert_eq!(game.sets[2].green, 10);
    assert_eq!(game.sets[2].blue, 6);
    assert_eq!(game.sets[2].red, 0);
    assert_eq!(game.sets[3].green, 11);
    assert_eq!(game.sets[3].blue, 5);
    assert_eq!(game.sets[3].red, 1);
}

#[test]
fn test_is_game_valid() {
    let cases = [
        ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true),
        ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true),
        ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false),
        ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", false),
        ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true),
    ];
    for case in cases.iter() {
        let game = parse_line(case.0);
        assert_eq!(is_game_valid(&game), case.1);
    }
}

#[test]
fn test_fewest_possible_cubes() {
    let cases = [
        ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 4, 2, 6),
        ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 1, 3, 4),
        ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 20, 13, 6),
        ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 14, 3, 15),
        ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 6, 3, 2),
    ];
    for case in cases.iter() {
        let game = parse_line(case.0);
        let fewest_possible_cubes = fewest_possible_cubes(&game);
        assert_eq!(fewest_possible_cubes.red, case.1);
        assert_eq!(fewest_possible_cubes.green, case.2);
        assert_eq!(fewest_possible_cubes.blue, case.3);
    }
}
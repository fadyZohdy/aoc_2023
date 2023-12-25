use crate::utils::get_input_file_name;
use std::fs::read_to_string;
use std::io::Error;

// Time:      7  15   30
// Distance:  9  40  200

fn parse_line(line: &str) -> Vec<i64> {
    line.split(":")
        .nth(1)
        .map(|s| s.split(" ").filter_map(|token| token.parse::<i64>().ok()))
        .expect("malformed line")
        .collect::<Vec<i64>>()
}

fn num_ways_to_win(time: i64, distance: i64) -> i64 {
    // t (time)
    // r (record)
    // h (hold time) 0..t
    // h * (t - h) > r
    // ht - h^2 > r
    // ht - h^2 - r > 0
    // - h^2 + th - r > 0
    // this is a quadratic formula that follows this equation  ax^2 + bx + c = 0
    // we need to solve for x
    let a = -1f64;
    let b = time as f64;
    let c = -distance as f64;
    let mut first_solution = ((-b + (b * b - 4. * a * c).sqrt()) / 2. * a).ceil();
    let mut second_solution = ((-b - (b * b - 4. * a * c).sqrt()) / 2. * a).floor();
    // now since we need to solve for > 0 not = 0, which means our solutions shouldn't be equal to
    // distance record;
    if first_solution * (b - first_solution) == distance as f64 {
        first_solution += 1.;
    }
    if second_solution * (b - second_solution) == distance as f64 {
        second_solution -= 1.;
    }
    (second_solution - first_solution) as i64 + 1
}

pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;

    let times = contents
        .lines()
        .nth(0)
        .map(parse_line)
        .expect("malformed input");
    let distances = contents
        .lines()
        .nth(1)
        .map(parse_line)
        .expect("malformed input");
    let res = times
        .iter()
        .zip(distances.clone())
        .fold(1, |acc, (t, d)| acc * num_ways_to_win(*t, d));
    println!("module: {}, part 1, result: {}", module_path!(), res);

    let time = times
        .iter()
        .map(|i| format!("{i}"))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = distances
        .iter()
        .map(|i| format!("{i}"))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let res2 = num_ways_to_win(time, distance);
    println!("module: {}, part 2, result: {}", module_path!(), res2);

    Ok(())
}

#[test]
fn test_num_ways_to_win() {
    assert_eq!(num_ways_to_win(7, 9), 4);
    assert_eq!(num_ways_to_win(15, 40), 8);
    assert_eq!(num_ways_to_win(30, 200), 9);
}

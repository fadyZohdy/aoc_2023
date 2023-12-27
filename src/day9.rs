use crate::utils::get_input_file_name;
use std::fs::read_to_string;
use std::io::Error;

fn solve_history(mut history: Vec<i64>) -> (i64, i64) {
    let mut first_values: Vec<i64> = vec![];

    let mut last_value = *history.last().unwrap();
    let mut first_value = history[0];
    first_values.push(first_value);
    while !history.iter().all(|x| *x == 0) {
        history = history
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        last_value += *history.last().unwrap();
        first_value = history[0];
        first_values.push(first_value);
    }
    (
        first_values.iter().rev().fold(0, |acc, e| e - acc),
        last_value,
    )
}
pub fn solve() -> Result<(), Error> {
    let input_file_name = get_input_file_name(module_path!());
    let contents = read_to_string(format!("src/{}", input_file_name))?;
    let solution: (i64, i64) = contents
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(' ').map(|t| t.parse().unwrap()).collect();
            solve_history(nums)
        })
        .fold((0, 0), |(acc_first, acc_last), (first, last)| {
            (acc_first + first, acc_last + last)
        });
    println!("module: {}, part 1, result: {}", module_path!(), solution.1);
    println!("module: {}, part 2, result: {}", module_path!(), solution.0);
    Ok(())
}

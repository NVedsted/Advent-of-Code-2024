pub mod algorithms;
mod answers;
mod data;
pub mod parsing;

use std::process::Termination;
use std::time::Instant;

pub fn execute_day<F, A1, A2>(day: usize, solver: F) -> impl Termination
where
    F: FnOnce(&str) -> (A1, A2),
    A1: ToString,
    A2: ToString,
{
    let input = data::get_day_input(day);
    let start = Instant::now();
    let answers = solver(&input);
    let elapsed = start.elapsed();

    let answers = (answers.0.to_string(), answers.1.to_string());

    let (answer1, answer2) = answers::validate_day_answers(day, answers);

    println!("{}", answer1);
    println!("{}", answer2);
    println!("Runtime: {elapsed:?}");
}

pub fn test_day<F: FnOnce(&str) -> (T1, T2), T1: ToString, T2: ToString>(
    solver: F,
    input: &str,
    expected: (Option<&str>, Option<&str>),
) {
    let (answer1, answer2) = answers::validate_answers(solver(input), expected);
    assert!(!answer1.is_incorrect(), "Part 1 failed");
    assert!(!answer2.is_incorrect(), "Part 2 failed");
}

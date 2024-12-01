pub mod algorithms;
mod answers;
mod data;
pub mod parsing;

use std::fmt::{Debug, Display};
use std::process::{ExitCode, Termination};
use std::time::Instant;

pub fn execute_day<F, A1, A2>(day: usize, solver: F) -> impl Termination
where
    F: FnOnce(&str) -> (A1, A2),
    A1: Display,
    A2: Display,
{
    let input = data::get_day_input(day);
    let start = Instant::now();
    let answers = solver(&input);
    let elapsed = start.elapsed();

    let answers = (answers.0, answers.1);

    let (answer1, answer2) = answers::validate_answers(day, answers);

    println!("{}", answer1);
    println!("{}", answer2);
    println!("Runtime: {elapsed:?}");

    if std::env::args().any(|a| a == "--save-output") {
        data::set_day_output(day, &answer1.value, &answer2.value);
    }

    if answer1.is_incorrect() || answer2.is_incorrect() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub fn test_day<F, T1, T2>(solver: F, input: &str, (expected1, expected2): (T1, Option<T2>))
where
    F: FnOnce(&str) -> (T1, T2),
    T1: PartialEq + Debug,
    T2: PartialEq + Debug,
{
    let (answer1, answer2) = solver(input);
    assert_eq!(answer1, expected1, "Part 1 failed");

    if let Some(expected2) = expected2 {
        assert_eq!(answer2, expected2, "Part 1 failed");
    }
}

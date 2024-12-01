use crate::data;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum AnswerState {
    Correct,
    Incorrect,
    Unvalidated,
}

impl Display for AnswerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnswerState::Correct => "CORRECT",
                AnswerState::Incorrect => "INCORRECT",
                AnswerState::Unvalidated => "UNVALIDATED",
            }
        )
    }
}

pub struct Answer<T> {
    value: T,
    state: AnswerState,
    part: usize,
}

impl<T> Answer<T> {
    pub fn is_incorrect(&self) -> bool {
        matches!(self.state, AnswerState::Incorrect)
    }
}

impl<T: Display> Display for Answer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part {}: {} ({})", self.part, self.value, self.state)
    }
}

fn validate_answer<T: ToString>(part: usize, value: T, expected: Option<&str>) -> Answer<T> {
    let state = if let Some(expected) = expected {
        if value.to_string().eq(expected) {
            AnswerState::Correct
        } else {
            AnswerState::Incorrect
        }
    } else {
        AnswerState::Unvalidated
    };

    Answer { value, state, part }
}

pub fn validate_answers<T1: ToString, T2: ToString>(
    (answer1, answer2): (T1, T2),
    (expected1, expected2): (Option<&str>, Option<&str>),
) -> (Answer<T1>, Answer<T2>) {
    (
        validate_answer(1, answer1, expected1),
        validate_answer(2, answer2, expected2),
    )
}

pub fn validate_day_answers<T1: ToString, T2: ToString>(
    day: usize,
    answer: (T1, T2),
) -> (Answer<T1>, Answer<T2>) {
    let (expected1, expected2) = data::get_day_output(day);
    validate_answers(answer, (expected1.as_deref(), expected2.as_deref()))
}

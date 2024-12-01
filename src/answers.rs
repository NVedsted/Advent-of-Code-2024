use crate::data;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
enum AnswerState {
    Correct,
    Incorrect(String),
    Unvalidated,
}

impl Display for AnswerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnswerState::Correct => write!(f, "CORRECT"),
            AnswerState::Incorrect(e) => write!(f, "INCORRECT; expected: {}", e),
            AnswerState::Unvalidated => write!(f, "UNVALIDATED"),
        }
    }
}

pub struct Answer {
    pub value: String,
    state: AnswerState,
    part: usize,
}

impl Answer {
    pub fn is_incorrect(&self) -> bool {
        matches!(self.state, AnswerState::Incorrect(_))
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part {}: {} ({})", self.part, self.value, self.state)
    }
}

fn validate_answer(part: usize, value: String, expected: Option<&str>) -> Answer {
    let state = if let Some(expected) = expected {
        if value.eq(expected) {
            AnswerState::Correct
        } else {
            AnswerState::Incorrect(expected.to_owned())
        }
    } else {
        AnswerState::Unvalidated
    };

    Answer { value, state, part }
}

pub fn validate_answers<T1: ToString, T2: ToString>(
    day: usize,
    (answer1, answer2): (T1, T2),
) -> (Answer, Answer) {
    let (expected1, expected2) = data::get_day_output(day);
    (
        validate_answer(1, answer1.to_string(), expected1.as_deref()),
        validate_answer(2, answer2.to_string(), expected2.as_deref()),
    )
}

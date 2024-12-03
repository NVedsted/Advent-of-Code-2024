use std::process::Termination;

const DO_INSTRUCTION: &str = "do()";

const DO_NOT_INSTRUCTION: &str = "don't()";

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct MulData {
    left: usize,
    right: usize,
}

impl MulData {
    fn compute(self) -> usize {
        self.left * self.right
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Mul(MulData),
    Do,
    DoNot,
}

impl Instruction {
    fn new_mul(left: usize, right: usize) -> Self {
        Self::Mul(MulData { left, right })
    }

    fn mul(&self) -> Option<MulData> {
        match self {
            Instruction::Mul(m) => Some(*m),
            _ => None,
        }
    }
}

fn parse_mul(input: &str) -> Option<(Instruction, usize)> {
    let (left, right) = input.strip_prefix("mul(")?.split_once(',')?;

    let left_len = left.len();
    let left = left.parse::<usize>().ok()?;
    let right_len = right.find(")")?;
    let right = right[..right_len].parse::<usize>().ok()?;

    Some((
        Instruction::new_mul(left, right),
        "mul(".len() + left_len + right_len + 2,
    ))
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut result = vec![];
    let mut remaining = input;

    while !remaining.is_empty() {
        if remaining.starts_with(DO_INSTRUCTION) {
            result.push(Instruction::Do);
            remaining = &remaining[DO_INSTRUCTION.len()..];
        } else if remaining.starts_with(DO_NOT_INSTRUCTION) {
            result.push(Instruction::DoNot);
            remaining = &remaining[DO_NOT_INSTRUCTION.len()..];
        } else if let Some((mul, consumed)) = parse_mul(remaining) {
            remaining = &remaining[consumed..];
            result.push(mul);
        } else {
            remaining = &remaining[1..];
        }
    }

    result
}

fn solver(input: &str) -> (usize, usize) {
    let instructions = parse(input);

    let part1 = instructions
        .iter()
        .filter_map(Instruction::mul)
        .map(MulData::compute)
        .sum();

    let (part2, _) = instructions
        .into_iter()
        .fold((0, true), |(sum, active), i| match i {
            Instruction::Mul(m) => (
                sum + active.then(|| m.compute()).unwrap_or_default(),
                active,
            ),
            Instruction::Do => (sum, true),
            Instruction::DoNot => (sum, false),
        });

    (part1, part2)
}

fn main() -> impl Termination {
    aoc2024::execute_day(3, solver)
}

#[cfg(test)]
mod tests {
    use crate::{parse, parse_mul, solver, Instruction};

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (161, Some(48)));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            [
                Instruction::new_mul(2, 4),
                Instruction::DoNot,
                Instruction::new_mul(5, 5),
                Instruction::new_mul(11, 8),
                Instruction::Do,
                Instruction::new_mul(8, 5)
            ]
        );
    }

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul(""), None);
        assert_eq!(parse_mul("xab"), None);
        assert_eq!(parse_mul("mul("), None);
        assert_eq!(parse_mul("mul(2,5)"), Some((Instruction::new_mul(2, 5), 8)));
        assert_eq!(
            parse_mul("mul(20,5)"),
            Some((Instruction::new_mul(20, 5), 9))
        );
        assert_eq!(
            parse_mul("mul(20,50)"),
            Some((Instruction::new_mul(20, 50), 10))
        );
        assert_eq!(
            parse_mul("mul(200,500)"),
            Some((Instruction::new_mul(200, 500), 12))
        );
    }
}

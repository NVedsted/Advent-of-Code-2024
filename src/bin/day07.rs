use std::process::Termination;
use std::str::FromStr;

struct Equation {
    expected: i64,
    numbers: Vec<i64>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (expected, numbers) = s.split_once(": ").unwrap();

        Ok(Self {
            expected: expected.parse().unwrap(),
            numbers: numbers.split(' ').map(|n| n.parse().unwrap()).collect(),
        })
    }
}

fn solver(input: &str) -> (i64, i64) {
    let equations = aoc2024::parsing::parse_list::<Equation>(input).collect::<Vec<_>>();

    fn compute_part<const PART2: bool>(equations: &[Equation]) -> i64 {
        equations
            .iter()
            .filter(|e| search_equation::<PART2>(e.expected, e.numbers[0], &e.numbers[1..]))
            .map(|e| e.expected)
            .sum()
    }

    (
        compute_part::<false>(&equations),
        compute_part::<true>(&equations),
    )
}

fn search_equation<const PART2: bool>(expected: i64, value: i64, numbers: &[i64]) -> bool {
    if let Some(next) = numbers.first().copied() {
        search_equation::<PART2>(expected, value + next, &numbers[1..])
            || search_equation::<PART2>(expected, value * next, &numbers[1..])
            || if PART2 {
                search_equation::<PART2>(expected, concatenate_numbers(value, next), &numbers[1..])
            } else {
                false
            }
    } else {
        expected == value
    }
}

fn concatenate_numbers(a: i64, b: i64) -> i64 {
    match (a, b) {
        (x, 0) | (0, x) => x,
        (a, b) => a * 10i64.pow(b.ilog10() + 1) + b,
    }
}

fn main() -> impl Termination {
    aoc2024::execute_day(7, solver)
}

#[cfg(test)]
mod tests {
    use crate::{concatenate_numbers, solver};

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (3749, Some(11387)));
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate_numbers(12, 345), 12345);
        assert_eq!(concatenate_numbers(1, 56), 156);
        assert_eq!(concatenate_numbers(1, 2), 12);
        assert_eq!(concatenate_numbers(0, 145), 145);
        assert_eq!(concatenate_numbers(168, 0), 168);
    }
}

use aoc2024::{algorithms, parsing};
use std::process::Termination;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    left: usize,
    right: usize,
}

impl Line {
    fn into_tuple(self) -> (usize, usize) {
        (self.left, self.right)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        Ok(Line {
            left: split.next().unwrap().parse().unwrap(),
            right: split.next().unwrap().parse().unwrap(),
        })
    }
}

fn solver(input: &str) -> (usize, usize) {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) =
        parsing::parse_list(input).map(Line::into_tuple).unzip();

    left_list.sort();
    right_list.sort();

    assert_eq!(left_list.len(), right_list.len());

    let part1 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<usize>();

    let part2 = left_list
        .into_iter()
        .map(|v| v * algorithms::binary_count(&right_list, &v))
        .sum::<usize>();

    (part1, part2)
}

fn main() -> impl Termination {
    aoc2024::execute_day(1, solver)
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    const EXAMPLE_OUTPUT: (Option<&str>, Option<&str>) = (Some("11"), Some("31"));

    #[test]
    fn test() {
        aoc2024::test_day(super::solver, EXAMPLE_INPUT, EXAMPLE_OUTPUT);
    }
}

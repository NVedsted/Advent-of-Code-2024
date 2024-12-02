use crate::ReportBehavior::{Decreasing, Either, Increasing};
use std::process::Termination;
use std::str::FromStr;

struct Report(Vec<usize>);

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report(
            s.split_whitespace().map(|d| d.parse().unwrap()).collect(),
        ))
    }
}

fn solver(input: &str) -> (usize, usize) {
    let reports = aoc2024::parsing::parse_list::<Report>(input).collect::<Vec<_>>();
    let part1 = reports.iter().filter(|r| is_safe(r)).count();
    let part2 = reports.into_iter().filter(is_safe_dampened).count();
    (part1, part2)
}

fn is_safe(report: &Report) -> bool {
    find_behavior(&report.0).is_some()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ReportBehavior {
    Increasing,
    Decreasing,
    Either,
}

impl ReportBehavior {
    fn from_pair(left: usize, right: usize) -> Self {
        if right > left {
            Increasing
        } else if right == left {
            Either
        } else {
            Decreasing
        }
    }
    fn intersection(self, other: Self) -> Option<Self> {
        match (self, other) {
            (l, r) if l == r => Some(l),
            (v, Either) | (Either, v) => Some(v),
            _ => None,
        }
    }

    fn validate_pair(self, left: usize, right: usize) -> bool {
        left != right
            && left.abs_diff(right) <= 3
            && match self {
                Increasing => right > left,
                Decreasing => right < left,
                Either => true,
            }
    }
}

fn find_behavior(values: &[usize]) -> Option<ReportBehavior> {
    if values.len() < 2 {
        return Some(Either);
    }

    let initial_behavior = ReportBehavior::from_pair(values[0], values[1]);

    if values
        .windows(2)
        .all(|v| initial_behavior.validate_pair(v[0], v[1]))
    {
        Some(initial_behavior)
    } else {
        None
    }
}

fn is_safe_dampened(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    let values = report.0.as_slice();

    (0..report.0.len()).any(|i| {
        let (left, right) = {
            let s = values.split_at(i);
            (s.0, &s.1[1..])
        };

        let (Some(left_behavior), Some(right_behavior)) =
            (find_behavior(left), find_behavior(right))
        else {
            return false;
        };

        let Some(common_behavior) = left_behavior.intersection(right_behavior) else {
            return false;
        };

        match (left.last(), right.first()) {
            (None, _) | (_, None) => true,
            (Some(&a), Some(&b)) => common_behavior.validate_pair(a, b),
        }
    })
}

fn main() -> impl Termination {
    aoc2024::execute_day(2, solver)
}

#[cfg(test)]
mod tests {
    use crate::{is_safe, is_safe_dampened, solver, Report};

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (2, Some(4)));
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&Report(vec![7, 6, 4, 2, 1])));
        assert!(!is_safe(&Report(vec![1, 2, 7, 8, 9])));
        assert!(!is_safe(&Report(vec![9, 7, 6, 2, 1])));
        assert!(!is_safe(&Report(vec![1, 3, 2, 4, 5])));
        assert!(!is_safe(&Report(vec![8, 6, 4, 4, 1])));
        assert!(is_safe(&Report(vec![1, 3, 6, 7, 9])));
    }

    #[test]
    fn test_is_safe_dampened() {
        assert!(is_safe_dampened(&Report(vec![7, 6, 4, 2, 1])));
        assert!(!is_safe_dampened(&Report(vec![1, 2, 7, 8, 9])));
        assert!(!is_safe_dampened(&Report(vec![9, 7, 6, 2, 1])));
        assert!(is_safe_dampened(&Report(vec![1, 3, 2, 4, 5])));
        assert!(is_safe_dampened(&Report(vec![8, 6, 4, 4, 1])));
        assert!(is_safe_dampened(&Report(vec![1, 3, 6, 7, 9])));
    }
}

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
    let values = &report.0;

    if values.len() < 2 {
        return true;
    }

    let increasing = values[1] > values[0];

    values.windows(2).all(|v| {
        v[0] != v[1]
            && v[0].abs_diff(v[1]) <= 3
            && if increasing { v[1] > v[0] } else { v[1] < v[0] }
    })
}

fn is_safe_dampened(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    let mut new_report = Report(Vec::with_capacity(report.0.len() - 1));

    (0..report.0.len()).any(|i| {
        new_report.0.clear();
        new_report.0.extend(&report.0[..i]);
        new_report.0.extend(&report.0[i + 1..]);
        is_safe(&new_report)
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

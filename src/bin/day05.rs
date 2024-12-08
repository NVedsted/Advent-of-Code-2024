use std::cmp::Ordering;
use std::collections::HashMap;
use std::process::Termination;

fn solver(input: &str) -> (usize, usize) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .flat_map(|l| {
            let (l, r) = l.split_once("|").unwrap();
            let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());

            [((l, r), Ordering::Less), ((r, l), Ordering::Greater)]
        })
        .collect::<HashMap<_, _>>();

    updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |(p1, p2), mut update| {
            assert_eq!(update.len() % 2, 1);
            if update
                .is_sorted_by(|&a, &b| matches!(rules[&(a, b)], Ordering::Less | Ordering::Equal))
            {
                (p1 + update[update.len() / 2], p2)
            } else {
                update.sort_by(|&a, &b| rules[&(a, b)]);
                (p1, p2 + update[update.len() / 2])
            }
        })
}

fn main() -> impl Termination {
    aoc2024::execute_day(5, solver)
}

#[cfg(test)]
mod tests {
    use crate::solver;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (143, Some(123)));
    }
}

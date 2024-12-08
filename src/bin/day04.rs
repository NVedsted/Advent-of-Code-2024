use std::ops::{Add, Mul};
use std::process::Termination;

const XMAS: &[u8; 4] = b"XMAS";
const MAS: &[u8; 3] = b"MAS";

#[derive(Copy, Clone)]
struct Position(usize, usize);

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Direction) -> Self::Output {
        Some(Self(
            (self.0 as i64).checked_add(rhs.0)? as usize,
            (self.1 as i64).checked_add(rhs.1)? as usize,
        ))
    }
}

#[derive(Copy, Clone)]
struct Direction(i64, i64);

impl Mul<usize> for Direction {
    type Output = Direction;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as i64, self.1 * rhs as i64)
    }
}

fn check<const N: usize>(
    input: &[&[u8]],
    pos: Position,
    dir: Direction,
    word: &[u8; N],
) -> bool {
    let mut letters: [u8; N] = [0; N];

    if letters
        .iter_mut()
        .enumerate()
        .try_for_each(|(i, l)| {
            let Position(r, c) = (pos + dir * i)?;
            *l = *input.get(r)?.get(c)?;
            Some(())
        })
        .is_none()
    {
        false
    } else {
        &letters == word || letters.iter().rev().eq(word)
    }
}

fn check_x(input: &[&[u8]], pos: Position) -> bool {
    if let (Some(left_corner), Some(right_corner)) =
        (pos + Direction(-1, -1), pos + Direction(-1, 1))
    {
        check(input, left_corner, Direction(1, 1), MAS)
            && check(input, right_corner, Direction(1, -1), MAS)
    } else {
        false
    }
}

fn solver(input: &str) -> (usize, usize) {
    let data = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let height = data.len();
    let width = data.first().expect("at least one row should be present").len();

    const PART1_DIRECTIONS: [Direction; 4] = [
        Direction(0, 1),
        Direction(1, 0),
        Direction(1, 1),
        Direction(1, -1),
    ];

    (0..height)
        .flat_map(|r| (0..width).map(move |c| Position(r, c)))
        .fold((0, 0), |(p1, p2), pos| {
            (
                p1 + PART1_DIRECTIONS
                    .iter()
                    .filter(|d| check(&data, pos, **d, XMAS))
                    .count(),
                p2 + if check_x(&data, pos) { 1 } else { 0 },
            )
        })
}

fn main() -> impl Termination {
    aoc2024::execute_day(4, solver)
}

#[cfg(test)]
mod tests {
    use crate::solver;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (18, Some(9)));
    }
}

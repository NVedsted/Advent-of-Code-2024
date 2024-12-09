use std::collections::HashSet;
use std::process::Termination;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn apply(&self, width: usize, height: usize, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
            Direction::East if pos.1 < width - 1 => Some((pos.0, pos.1 + 1)),
            Direction::South if pos.0 < height - 1 => Some((pos.0 + 1, pos.1)),
            Direction::West if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
            _ => None,
        }
    }
}

fn walk(
    position: (usize, usize),
    width: usize,
    height: usize,
    walls: &HashSet<(usize, usize)>,
) -> impl Iterator<Item = ((usize, usize), Direction)> + use<'_> {
    std::iter::successors(Some((position, Direction::North)), move |&(pos, dir)| {
        let ahead = dir.apply(width, height, pos)?;
        if walls.contains(&ahead) {
            Some((pos, dir.rotate_clockwise()))
        } else {
            Some((ahead, dir))
        }
    })
}

fn count_positions(
    position: (usize, usize),
    width: usize,
    height: usize,
    walls: &HashSet<(usize, usize)>,
) -> usize {
    let visited = walk(position, width, height, walls)
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    visited.len()
}

fn count_loops(
    position: (usize, usize),
    width: usize,
    height: usize,
    walls: &HashSet<(usize, usize)>,
) -> usize {
    let mut new_walls = walls.clone();
    let mut visited = HashSet::new();

    walk(position, width, height, walls)
        .filter_map(|(p, d)| d.apply(width, height, p))
        .filter(|&(r, c)| {
            if !walls.contains(&(r, c)) && visited.insert((r, c)) {
                new_walls.insert((r, c));
                let loops = loops(position, width, height, &new_walls);
                new_walls.remove(&(r, c));
                loops
            } else {
                false
            }
        })
        .count()
}

fn loops(
    position: (usize, usize),
    width: usize,
    height: usize,
    walls: &HashSet<(usize, usize)>,
) -> bool {
    let mut visited_states = HashSet::new();
    walk(position, width, height, walls).any(|s| !visited_states.insert(s))
}

fn solver(input: &str) -> (usize, usize) {
    let position = input
        .lines()
        .enumerate()
        .find_map(|(r, row)| row.find('^').map(|c| (r, c)))
        .unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let walls = input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, t)| t == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect::<HashSet<_>>();

    (
        count_positions(position, width, height, &walls),
        count_loops(position, width, height, &walls),
    )
}

fn main() -> impl Termination {
    aoc2024::execute_day(6, solver)
}

#[cfg(test)]
mod tests {
    use crate::solver;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_day() {
        aoc2024::test_day(solver, EXAMPLE_INPUT, (41, Some(6)));
    }
}

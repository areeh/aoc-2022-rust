extern crate test;

use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

use itertools::Itertools;
use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::{pretty_print, read_input_to_string};

fn parse_input(input: &str) -> Vec<Position> {
    let mut elves = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Position::new(col as i64, row as i64));
            }
        }
    }
    elves
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn to_index(self) -> [usize; 2] {
        [self.y.try_into().unwrap(), self.x.try_into().unwrap()]
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::NorthWest => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::SouthWest => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::SouthEast => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::NorthEast => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn check_directions(dir: &Direction) -> [Direction; 3] {
    match dir {
        Direction::North => [Direction::NorthWest, Direction::North, Direction::NorthEast],
        Direction::West => [Direction::SouthWest, Direction::West, Direction::NorthWest],
        Direction::South => [Direction::SouthWest, Direction::South, Direction::SouthEast],
        Direction::East => [Direction::SouthEast, Direction::East, Direction::NorthEast],
        _ => panic!("Unexpected check direction {dir:?}"),
    }
}

fn neighbors(
    pos: &Position,
    check_order: &[Direction; 4],
    occupancy: &HashSet<Position>,
) -> [(Position, bool); 12] {
    let mut ret = Vec::with_capacity(8);
    for dir in check_order {
        for check in check_directions(dir) {
            let check_pos = *pos + check;
            ret.push((check_pos, occupancy.contains(&check_pos)));
        }
    }
    ret.try_into().unwrap()
}

fn get_proposed_position(pos: &Position, neighbors: [(Position, bool); 12]) -> Position {
    if neighbors.iter().all(|(_, occupied)| !occupied) {
        *pos
    } else if let Some(move_pos) =
        neighbors
            .iter()
            .tuple_windows()
            .step_by(3)
            .find_map(|(l, m, r)| {
                if !(l.1 || m.1 || r.1) {
                    Some(m.0)
                } else {
                    None
                }
            })
    {
        move_pos
    } else {
        *pos
    }
}

fn min_max_per_axis<'a>(positions: impl Iterator<Item = &'a Position>) -> (Position, Position) {
    positions.fold(
        (Position::new(i64::MAX, i64::MAX), Position::new(0, 0)),
        |(mut mn, mut mx), pos| {
            if pos.x > mx.x {
                mx.x = pos.x;
            }
            if pos.y > mx.y {
                mx.y = pos.y;
            }

            if pos.x < mn.x {
                mn.x = pos.x;
            }
            if pos.y < mn.y {
                mn.y = pos.y;
            }

            (mn, mx)
        },
    )
}

fn visualize(elves: &[Position]) -> String {
    let (mn, mx) = min_max_per_axis(elves.iter());
    let shape = mx - mn + Position::new(1, 1);
    let mut arr: Array2<char> = Array2::<char>::from_elem(shape.to_index(), '.');
    for pos in elves {
        arr[(*pos - mn).to_index()] = '#';
    }
    pretty_print(&arr)
}

#[allow(dead_code)]
fn visualize_print(elves: &[Position]) {
    println!("{}", visualize(elves));
}

fn parts(input: &str, max_rounds: Option<usize>) -> (Vec<Position>, usize) {
    let mut check_order = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut elves = parse_input(input);
    let mut elves_occupancy: HashSet<Position> = elves.clone().into_iter().collect();
    let mut next_elves: Vec<Position> = elves.clone().into_iter().collect();
    let mut proposed_positions: HashSet<Position> = HashSet::with_capacity(elves.capacity());
    let mut duplicated: HashSet<Position> = HashSet::new();
    let mut changed = true;

    let mut round = 0;

    loop {
        if !changed {
            break;
        }

        for i in 0..elves.len() {
            let pos = elves[i];
            let next_pos =
                get_proposed_position(&pos, neighbors(&pos, &check_order, &elves_occupancy));
            if proposed_positions.contains(&next_pos) {
                duplicated.insert(next_pos);
            } else {
                proposed_positions.insert(next_pos);
            }
            next_elves[i] = next_pos;
        }

        changed = false;
        for i in 0..elves.len() {
            if !duplicated.contains(&next_elves[i]) && next_elves[i] != elves[i] {
                changed = true;
                elves[i] = next_elves[i];
            }
        }
        elves_occupancy = elves.clone().into_iter().collect();
        check_order.rotate_left(1);

        proposed_positions.clear();
        duplicated.clear();
        round += 1;
        if let Some(mx) = max_rounds {
            if round >= mx {
                break;
            }
        }
    }
    (elves, round)
}

fn part1(input: &str) -> usize {
    let (elves, _) = parts(input, Some(10));
    let (mn, mx) = min_max_per_axis(elves.iter());
    let rectangle = mx - mn + Position::new(1, 1);
    (rectangle.x * rectangle.y) as usize - elves.len()
}

fn part2(input: &str) -> usize {
    parts(input, None).1
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(23)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn tiny_example() {
    let input = ".....
..##.
..#..
.....
..##.
.....";
    assert_eq!(part1(input), 25);
    assert_eq!(part2(input), 4);
}

#[test]
fn example() {
    let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    assert_eq!(part1(input), 110);
    assert_eq!(part2(input), 20);
}

#[test]
fn task() {
    let input = &read_input_to_string(23).unwrap();
    assert_eq!(part1(input), 4005);
    // assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(23).unwrap();
        part1(input);
        part2(input);
    })
}

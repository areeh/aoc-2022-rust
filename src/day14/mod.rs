extern crate test;

use std::{
    iter::once,
    ops::{Add, Sub},
};

use itertools::Itertools;
use ndarray::{s, Array2};
#[cfg(test)]
use test::Bencher;

use crate::utils::{pretty_print, read_input_to_string};

fn pretty_print_swap(arr: &Array2<char>) -> String {
    let mut arr = arr.clone();
    arr.swap_axes(0, 1);
    pretty_print(&arr)
}

fn parse_line(line: &str) -> Vec<Position> {
    line.split(" -> ").map(Position::from).collect()
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input.lines().map(parse_line).collect()
}

fn draw_path(path: &[Position], map: &mut Array2<char>, min_pos: &Position) {
    #[allow(clippy::collapsible_else_if)]
    path.iter().tuple_windows().for_each(|(l, r)| {
        let l = *l - *min_pos;
        let r = *r - *min_pos;
        if l.0 == r.0 {
            if l.1 > r.1 {
                map.slice_mut(s![l.0, r.1..l.1 + 1]).fill('#');
            } else {
                map.slice_mut(s![l.0, l.1..r.1 + 1]).fill('#');
            }
        } else {
            if l.0 > r.0 {
                map.slice_mut(s![r.0..l.0 + 1, r.1]).fill('#');
            } else {
                map.slice_mut(s![l.0..r.0 + 1, l.1]).fill('#');
            }
        }
    })
}

fn draw_paths(paths: &[Vec<Position>], map: &mut Array2<char>, min_pos: &Position) {
    paths.iter().for_each(|path| draw_path(path, map, min_pos));
}

fn min_max_per_axis<'a>(positions: impl Iterator<Item = &'a Position>) -> (Position, Position) {
    positions.fold(
        (Position(usize::MAX, usize::MAX), Position(0, 0)),
        |(mut mn, mut mx), pos| {
            if pos.0 > mx.0 {
                mx.0 = pos.0;
            }
            if pos.1 > mx.1 {
                mx.1 = pos.1;
            }

            if pos.0 < mn.0 {
                mn.0 = pos.0;
            }
            if pos.1 < mn.1 {
                mn.1 = pos.1;
            }

            (mn, mx)
        },
    )
}

enum Offset {
    Neg(usize),
    Pos(usize),
}

impl Offset {
    fn add_to_usize(self, v: usize) -> Option<usize> {
        match self {
            Offset::Pos(offset) => v.checked_add(offset),
            Offset::Neg(offset) => v.checked_sub(offset),
        }
    }
}

enum Direction {
    Down,
    DownLeft,
    DownRight,
}

impl Direction {
    fn delta(&self) -> (Offset, Offset) {
        match self {
            Direction::Down => (Offset::Pos(0), Offset::Pos(1)),
            Direction::DownLeft => (Offset::Neg(1), Offset::Pos(1)),
            Direction::DownRight => (Offset::Pos(1), Offset::Pos(1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn to_index(self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn checked_add(self, dir: Direction) -> Option<Position> {
        let dir_delta = dir.delta();
        let l = dir_delta.0.add_to_usize(self.0);
        let r = dir_delta.1.add_to_usize(self.1);
        match (l, r) {
            (Some(l), Some(r)) => Some(Position(l, r)),
            _ => None,
        }
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let (y, x) = s
            .split_once(',')
            .unwrap_or_else(|| panic!("Unexpected string for Position {s}"));
        Position(y.parse().unwrap(), x.parse().unwrap())
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        let dir_delta = dir.delta();
        Position(
            dir_delta.0.add_to_usize(self.0).unwrap(),
            dir_delta.1.add_to_usize(self.1).unwrap(),
        )
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(
            self.0.checked_sub(other.0).unwrap(),
            self.1.checked_sub(other.1).unwrap(),
        )
    }
}

struct Cave {
    start: Position,
    map: Array2<char>,
}

impl Cave {
    fn new(input: &str, p2: bool) -> Self {
        let path = parse_input(input);
        let start = Position(500, 0);
        let (mut mn, mut mx) = min_max_per_axis(path.iter().flatten().chain(once(&start)));
        if p2 {
            mx = mx + Position(0, 2);
            mx = mx + Position(mx.1 - 2, 0);
            mn = mn - Position(mx.1 - 5, 0);
        }
        let shape = (mx - mn) + Position(1, 1);
        let mut map = Array2::from_elem(shape.to_index(), '.');
        draw_paths(&path, &mut map, &mn);

        if p2 {
            map.slice_mut(s![0..map.dim().0, map.dim().1 - 1]).fill('#');
        }

        let start = start - mn;

        Cave { start, map }
    }

    fn visualize(&self) -> String {
        let mut map = self.map.clone();
        if map[self.start.to_index()] != 'o' {
            map[self.start.to_index()] = '+';
        }
        pretty_print_swap(&map)
    }

    #[allow(dead_code)]
    fn print_visualize(&self) {
        println!("{}", self.visualize())
    }

    /// true if we landed, false otherwise
    fn drop_sand(&mut self) -> bool {
        if let Some(v) = self.map.get(self.start.to_index()) {
            if *v == 'o' {
                return false;
            }
        }

        let mut sand_pos = Some(self.start);
        let mut next_pos = None;
        loop {
            for dir in [Direction::Down, Direction::DownLeft, Direction::DownRight] {
                if let Some(try_pos) = sand_pos.unwrap().checked_add(dir) {
                    next_pos = match self.map.get(try_pos.to_index()) {
                        Some('.') => Some(try_pos),
                        Some('#') => None,
                        Some('o') => None,
                        Some(c) => panic!("Unknown map character {c}"),
                        None => return false,
                    };
                    if next_pos.is_some() {
                        break;
                    }
                } else {
                    return false;
                }
            }
            if next_pos.is_some() {
                sand_pos = next_pos;
            } else {
                break;
            }
        }
        self.map[sand_pos.unwrap().to_index()] = 'o';
        true
    }
}

fn part1(input: &str) -> usize {
    let mut cave = Cave::new(input, false);
    let mut i = 0;
    while cave.drop_sand() {
        i += 1;
    }
    i
}

fn part2(input: &str) -> usize {
    let mut cave = Cave::new(input, true);
    let mut i = 0;
    while cave.drop_sand() {
        i += 1;
    }
    i
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(14)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_parse_line() {
    let input = "498,4 -> 498,6 -> 496,6";
    assert_eq!(
        parse_line(input),
        vec![Position(498, 4), Position(498, 6), Position(496, 6)]
    );
}

#[test]
fn example_visualize() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let cave = Cave::new(input, false);
    assert_eq!(
        cave.visualize(),
        "
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
"
        .trim()
    );
}

#[test]
fn example_visualize_drop() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let mut cave = Cave::new(input, false);
    for _ in 0..24 {
        cave.drop_sand();
    }
    assert_eq!(
        cave.visualize(),
        "
......+...
..........
......o...
.....ooo..
....#ooo##
...o#ooo#.
..###ooo#.
....oooo#.
.o.ooooo#.
#########.
"
        .trim()
    );
}

#[test]
fn example_visualize_drop_p2() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let mut cave = Cave::new(input, true);
    for _ in 0..93 {
        cave.drop_sand();
    }
    cave.print_visualize();
    assert_eq!(
        cave.visualize(),
        "
............o............
...........ooo...........
..........ooooo..........
.........ooooooo.........
........oo#ooo##o........
.......ooo#ooo#ooo.......
......oo###ooo#oooo......
.....oooo.oooo#ooooo.....
....oooooooooo#oooooo....
...ooo#########ooooooo...
..ooooo.......ooooooooo..
#########################
"
        .trim()
    );
}

#[test]
fn example() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn task() {
    let input = &read_input_to_string(14).unwrap();
    assert_eq!(part1(input), 674);
    assert_eq!(part2(input), 24958);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(14).unwrap();
        part1(input);
        part2(input);
    })
}

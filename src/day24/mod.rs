extern crate test;

use std::ops::{Add, AddAssign, Sub};

use itertools::Itertools;
use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::{pretty_print, read_input_to_string};

type Board = Array2<char>;

fn parse_board(input: &str) -> Array2<char> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line.trim().chars().collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;

    let mut board = Array2::from_shape_vec((n_rows, board_width), data).unwrap();
    board.mapv_inplace(|v| {
        if ['<', '>', '^', 'v'].contains(&v) {
            '.'
        } else {
            v
        }
    });
    board
}

fn parse_blizzards(input: &str) -> Vec<(Position, Direction)> {
    let mut blizzards = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(dir) = char_to_dir(c) {
                blizzards.push((Position::new(col, row), dir));
            }
        }
    }
    blizzards
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::Up),
        '<' => Some(Direction::Left),
        'v' => Some(Direction::Down),
        '>' => Some(Direction::Right),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

enum Action {
    Move(Direction),
    Wait,
}

const ACTIONS: [Action; 5] = [
    Action::Move(Direction::Right),
    Action::Move(Direction::Down),
    Action::Move(Direction::Left),
    Action::Move(Direction::Up),
    Action::Wait,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_index(self) -> [usize; 2] {
        [self.y, self.x]
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
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

fn visualize(blizzards: &[(Position, Direction)], board: &Board) -> String {
    let mut board = board.clone();

    for (pos, dir) in blizzards {
        board[pos.to_index()] = match dir {
            Direction::Up => '^',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Right => '>',
        };
    }
    pretty_print(&board)
}

fn wrap_position(pos: &Position, dir: &Direction, board: &Board) -> Position {
    match dir {
        Direction::Up => Position {
            x: pos.x,
            y: board.dim().0 - 2,
        },
        Direction::Left => Position {
            x: board.dim().1 - 2,
            y: pos.y,
        },
        Direction::Down => Position { x: pos.x, y: 1 },
        Direction::Right => Position { x: 1, y: pos.y },
    }
}

fn step_blizzards(blizzards: &mut [(Position, Direction)], board: &Board) {
    for (pos, dir) in blizzards.iter_mut() {
        let next = *pos + *dir;
        let value_at_next = board.get(next.to_index()).unwrap();
        match value_at_next {
            '.' => *pos = next,
            '#' => *pos = wrap_position(pos, dir, board),
            _ => panic!("Unexpected board value {value_at_next}"),
        }
        // visualize_print(&board, pos, facing);
    }
}

fn part1(input: &str) {}

fn part2(input: &str) {}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(24)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn tiny_example() {
    let input = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";
    let board = parse_board(input);
    let mut blizzards = parse_blizzards(input);
    for _ in 0..5 {
        step_blizzards(&mut blizzards, &board);
    }
    println!("{}", visualize(&blizzards, &board))
}

#[test]
fn example() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    assert_eq!(part1(input), ());
    assert_eq!(part2(input), ());
}

#[test]
fn task() {
    let input = &read_input_to_string(24).unwrap();
    assert_eq!(part1(input), ());
    assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(24).unwrap();
        part1(input);
        part2(input);
    })
}

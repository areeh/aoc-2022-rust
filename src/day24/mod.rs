extern crate test;

use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

use itertools::Itertools;
use ndarray::{s, Array2, Dim};
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
                blizzards.push((Position::new(col + 1, row + 1), dir));
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

    fn manhattan(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
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

#[allow(dead_code)]
fn visualize(
    board: &Board,
    blizzards: &[(Position, Direction)],
    expedition: Option<&Position>,
    goal: Option<&Position>,
) -> String {
    let mut board = board.clone();

    for (pos, dir) in blizzards {
        board[pos.to_index()] = match dir {
            Direction::Up => '^',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Right => '>',
        };
    }
    if let Some(pos) = goal {
        board[pos.to_index()] = 'G';
    }
    if let Some(pos) = expedition {
        board[pos.to_index()] = 'E';
    }
    pretty_print(&board)
}

#[allow(dead_code)]
fn visualize_print(
    board: &Board,
    blizzards: &[(Position, Direction)],
    expedition: Option<&Position>,
    goal: Option<&Position>,
) {
    println!("{}", visualize(board, blizzards, expedition, goal));
}

fn wrap_position(pos: &Position, dir: &Direction, board: &Board) -> Position {
    match dir {
        Direction::Up => Position {
            x: pos.x,
            y: board.dim().0 - 3,
        },
        Direction::Left => Position {
            x: board.dim().1 - 3,
            y: pos.y,
        },
        Direction::Down => Position { x: pos.x, y: 2 },
        Direction::Right => Position { x: 2, y: pos.y },
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
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    minute: usize,
}

impl State {
    fn get_next_states(
        &self,
        next_blizzard_positions: &HashSet<Position>,
        board: &Board,
    ) -> Vec<Self> {
        let mut next_states = Vec::new();
        for action in ACTIONS {
            let next_pos = match action {
                Action::Wait => self.position,
                Action::Move(dir) => self.position + dir,
            };
            let value_at_next = board.get(next_pos.to_index()).unwrap();
            if *value_at_next == '.' && !next_blizzard_positions.contains(&next_pos) {
                next_states.push(State {
                    position: next_pos,
                    minute: self.minute + 1,
                })
            }
        }

        next_states
    }
}

fn pad(arr: &Board, value: char) -> Board {
    // janky pad implementation
    let mut board = Array2::from_elem(arr.raw_dim() + Dim([2, 2]), '#');
    board.fill(value);
    board
        .slice_mut(s![1..board.shape()[0] - 1, 1..board.shape()[1] - 1])
        .assign(arr);
    board
}

fn pathfind(
    start: &Position,
    goal: &Position,
    blizzards: &mut [(Position, Direction)],
    board: &Board,
) -> usize {
    let mut states = vec![State {
        position: *start,
        minute: 0,
    }];
    let mut visited = HashSet::new();
    let mut minute = 0;

    while states[0].position.manhattan(goal) != 0 {
        minute += 1;
        step_blizzards(blizzards, board);
        let next_blizzard_positions: HashSet<_> = blizzards.iter().map(|(pos, _)| *pos).collect();
        let mut next_states = Vec::new();
        for state in states.iter() {
            let tmp = state.get_next_states(&next_blizzard_positions, board);
            next_states.extend(tmp.clone().into_iter().filter(|v| !visited.contains(v)));
            visited.extend(tmp.into_iter());
        }

        next_states.sort_by(|a, b| a.position.manhattan(goal).cmp(&b.position.manhattan(goal)));

        // visualize_print(&board, &blizzards, Some(&next_states[0].position), None);
        // println!();

        next_states = next_states.into_iter().take(100).collect_vec();

        states = next_states
    }
    minute
}

fn part1(input: &str) -> usize {
    let board = parse_board(input);
    let board = pad(&board, '#');
    let mut blizzards = parse_blizzards(input);
    let start = Position::new(2, 1);
    let goal = Position::new(board.dim().1 - 3, board.dim().0 - 2);

    // visualize_print(&board, &blizzards, Some(&start), Some(&goal));

    pathfind(&start, &goal, &mut blizzards, &board)
}

fn part2(input: &str) -> usize {
    let board = parse_board(input);
    let board = pad(&board, '#');
    let mut blizzards = parse_blizzards(input);
    let start = Position::new(2, 1);
    let goal = Position::new(board.dim().1 - 3, board.dim().0 - 2);

    // visualize_print(&board, &blizzards, Some(&start), Some(&goal));

    pathfind(&start, &goal, &mut blizzards, &board)
        + pathfind(&goal, &start, &mut blizzards, &board)
        + pathfind(&start, &goal, &mut blizzards, &board)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(24)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn tiny_example() {
    let input = "
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#"
        .trim();
    let board = parse_board(input);
    let mut blizzards = parse_blizzards(input);
    for _ in 0..4 {
        step_blizzards(&mut blizzards, &board);
    }
    assert_eq!(
        visualize(&board, &blizzards, None, None),
        "
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#"
            .trim()
    )
}

#[test]
fn example() {
    let input = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
        .trim();
    assert_eq!(part1(input), 18);
    assert_eq!(part2(input), 54);
}

#[test]
fn task() {
    let input = &read_input_to_string(24).unwrap();
    assert_eq!(part1(input), 253);
    assert_eq!(part2(input), 794);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(24).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

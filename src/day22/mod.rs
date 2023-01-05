extern crate test;

use std::{
    iter::zip,
    ops::{Add, AddAssign, Sub},
    str::Chars,
};

use bimap::BiMap;
use itertools::Itertools;
use ndarray::{s, Array2, Dim};
#[cfg(test)]
use test::Bencher;

use crate::utils::{has_unique_elements, pretty_print, read_input_to_string};

type Board = Array2<char>;

fn parse_board(board: &str) -> Board {
    let board_width = board.lines().map(|line| line.len()).max().unwrap();
    let n_rows = board.lines().count();

    let mut data = Vec::new();
    for line in board.lines() {
        let row: Vec<_> = line.chars().collect_vec();
        data.push(row);
    }
    let mut board = Array2::from_elem((n_rows, board_width), ' ');

    for (row_data, row) in zip(data.into_iter(), board.rows_mut()) {
        for (insert_v, v) in zip(row_data, row) {
            *v = insert_v;
        }
    }

    board
}

fn next_start(col: usize, row: usize, cube_dim: usize, input_lines: &[&str]) -> (usize, usize) {
    let mut col = col;
    let mut row = row;

    loop {
        if let Some(line) = input_lines.get(row) {
            if let Some(start) = line.chars().nth(col) {
                if start != ' ' {
                    return (col, row);
                } else {
                    col += cube_dim;
                }
            } else {
                col = 0;
                row += cube_dim;
            }
        } else {
            panic!("Ran off the edge of the input_lines")
        }
    }
}

/// Squares of the cube are collected in the order left to right then top to bottom
fn parse_cube_board(board: &str) -> ([Board; 6], [(usize, usize); 6]) {
    let cube_dim = board
        .lines()
        .map(|line| line.chars().filter(|v| *v != ' ').count())
        .min()
        .unwrap();
    let mut faces: Vec<Vec<char>> = Vec::new();

    let input_lines = board.lines().collect_vec();
    let mut starts = Vec::new();

    let mut col = 0;
    let mut row = 0;

    for _ in 0..6 {
        (col, row) = next_start(col, row, cube_dim, &input_lines);
        starts.push((col, row));

        // dbg!(col, row);

        let mut next_board = Vec::new();
        for data_row in input_lines.iter().skip(row).take(cube_dim) {
            next_board.append(&mut data_row[col..col + cube_dim].chars().collect_vec())
        }
        faces.push(next_board);
        col += cube_dim;
    }

    let mut ret: Vec<Board> = Vec::new();
    for board in faces {
        let board = Array2::from_shape_vec((cube_dim, cube_dim), board).unwrap();
        let board = pad(&board, ' ');
        ret.push(board);
    }
    (ret.try_into().unwrap(), starts.try_into().unwrap())
}

fn parse_input(input: &str) -> (Board, &str) {
    let (board, path) = input.split_once("\n\n").unwrap();

    let board = parse_board(board);
    (board, path.trim())
}

fn pad(arr: &Board, value: char) -> Board {
    // janky pad implementation
    let mut board = Array2::from_elem(arr.raw_dim() + Dim([2, 2]), ' ');
    board.fill(value);
    board
        .slice_mut(s![1..board.shape()[0] - 1, 1..board.shape()[1] - 1])
        .assign(arr);
    board
}

fn euclidian_mod(v: i64, len: usize) -> usize {
    let len = len as i64;
    if v < 0 {
        (((v % len) + len) % len) as usize
    } else {
        (v % len) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

impl Direction {
    fn turn(&mut self, direction: char) {
        let curr_dir_pos = DIRECTIONS.iter().position(|v| *v == *self).unwrap();
        let delta = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!("Unknown turn direction {direction}"),
        };
        *self = DIRECTIONS[euclidian_mod(curr_dir_pos as i64 + delta, DIRECTIONS.len())];
    }

    fn value(&self) -> usize {
        DIRECTIONS.iter().position(|v| *v == *self).unwrap()
    }
}

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

fn visualize(board: &Board, pos: Position, facing: Direction) -> String {
    let mut board = board.clone();
    board[pos.to_index()] = match facing {
        Direction::Up => '^',
        Direction::Left => '<',
        Direction::Down => 'v',
        Direction::Right => '>',
    };
    pretty_print(&board)
}

#[allow(dead_code)]
fn visualize_print(board: &Board, pos: Position, facing: Direction) {
    println!("{}", visualize(board, pos, facing));
}

enum Action {
    Move(usize),
    Turn(char),
}

struct Path<'a> {
    iter: Chars<'a>,
}

impl<'a> Path<'a> {
    fn new(path: &'a str) -> Self {
        Self { iter: path.chars() }
    }

    fn get_next(&mut self) -> Option<Action> {
        if let Some(c) = self.iter.clone().next() {
            match c {
                '0'..='9' => {
                    let str = self.iter.as_str();
                    while self.iter.clone().next().map_or(false, |ch| ch.is_numeric()) {
                        self.iter.next();
                    }
                    Some(Action::Move(
                        str[..str.len() - self.iter.as_str().len()].parse().unwrap(),
                    ))
                }
                _ => Some(Action::Turn(self.iter.next().unwrap())),
            }
        } else {
            None
        }
    }
}

fn wrap_position(pos: Position, facing: Direction, board: &Board) -> Option<Position> {
    let mut tmp_facing = facing;
    tmp_facing.turn('L');
    tmp_facing.turn('L');

    let mut tmp_pos = pos;

    while board[tmp_pos.to_index()] != ' ' {
        tmp_pos += tmp_facing;
    }

    // Step back one to find landing position
    let final_pos = tmp_pos + facing;
    if board[final_pos.to_index()] == '#' {
        None
    } else {
        Some(final_pos)
    }
}

fn password(pos: Position, facing: Direction) -> usize {
    1000 * pos.y + 4 * pos.x + facing.value()
}

fn part1(input: &str) -> usize {
    let (board, path) = parse_input(input);
    let board = pad(&board, ' ');

    let initial_pos = board.indexed_iter().find(|(_, v)| **v == '.').unwrap().0;
    let mut pos = Position::new(initial_pos.1, initial_pos.0);
    let mut facing = Direction::Right;

    // visualize_print(&board, pos, facing);

    let mut path = Path::new(path);

    while let Some(action) = path.get_next() {
        match action {
            Action::Move(distance) => {
                for _ in 0..distance {
                    let next = pos + facing;
                    let value_at_next = board.get(next.to_index()).unwrap();
                    match value_at_next {
                        '.' => pos = next,
                        '#' => break,
                        ' ' => {
                            if let Some(next) = wrap_position(pos, facing, &board) {
                                pos = next;
                            } else {
                                break;
                            }
                        }
                        _ => panic!("Unexpected board value {value_at_next}"),
                    }
                }
            }
            Action::Turn(dir) => facing.turn(dir),
        }
        // visualize_print(&board, pos, facing);
    }
    password(pos, facing)
}

fn wrap_position_cube(
    pos: Position,
    facing: Direction,
    boards: &[Board; 6],
    current_board: usize,
    transition_table: &Transitions,
) -> Option<(usize, Position, Direction)> {
    let cube_dim = boards[0].dim().0;
    let board_min = 1;
    let board_max = cube_dim - 2;
    let (new_board_number, enter_side) = get_transition((current_board, facing), transition_table);
    let new_pos = match (facing, enter_side) {
        (Direction::Up, Direction::Up) => Position::new(invert_index(pos.x, cube_dim), board_min),
        (Direction::Up, Direction::Left) => Position::new(board_min, pos.x),
        (Direction::Up, Direction::Down) => Position::new(pos.x, board_max),
        (Direction::Up, Direction::Right) => {
            Position::new(board_max, invert_index(pos.x, cube_dim))
        }

        (Direction::Left, Direction::Up) => Position::new(pos.y, board_min),
        (Direction::Left, Direction::Left) => {
            Position::new(board_min, invert_index(pos.y, cube_dim))
        }
        (Direction::Left, Direction::Down) => {
            Position::new(invert_index(pos.y, cube_dim), board_max)
        }
        (Direction::Left, Direction::Right) => Position::new(board_max, pos.y),

        (Direction::Down, Direction::Up) => Position::new(pos.x, board_min),
        (Direction::Down, Direction::Left) => {
            Position::new(board_min, invert_index(pos.x, cube_dim))
        }
        (Direction::Down, Direction::Down) => {
            Position::new(invert_index(pos.x, cube_dim), board_max)
        }
        (Direction::Down, Direction::Right) => Position::new(board_max, pos.x),

        (Direction::Right, Direction::Up) => {
            Position::new(invert_index(pos.y, cube_dim), board_min)
        }
        (Direction::Right, Direction::Left) => Position::new(board_min, pos.y),
        (Direction::Right, Direction::Down) => Position::new(pos.y, board_max),
        (Direction::Right, Direction::Right) => {
            Position::new(board_max, invert_index(pos.y, cube_dim))
        }
    };

    // println!("Came from {pos:?} {facing:?} ended on position {new_pos:?} {enter_side:?}");

    if boards[new_board_number][new_pos.to_index()] == '#' {
        return None;
    } else if boards[new_board_number][new_pos.to_index()] == ' ' {
        panic!("Landed on empty in new board.")
    }

    let mut new_facing = enter_side;
    new_facing.turn('L');
    new_facing.turn('L');

    Some((new_board_number, new_pos, new_facing))
}

type Transitions = BiMap<(usize, Direction), (usize, Direction)>;

fn make_transition_table(example: bool) -> Transitions {
    let mut transition_table: BiMap<(usize, Direction), (usize, Direction)> = BiMap::new();

    // Transition table according to manualy folding the squares in the input into a cube
    if example {
        transition_table.insert((0, Direction::Up), (1, Direction::Up));
        transition_table.insert((0, Direction::Left), (2, Direction::Up));
        transition_table.insert((0, Direction::Down), (3, Direction::Up));
        transition_table.insert((0, Direction::Right), (5, Direction::Right));

        transition_table.insert((1, Direction::Left), (5, Direction::Down));
        transition_table.insert((1, Direction::Down), (4, Direction::Down));
        transition_table.insert((1, Direction::Right), (2, Direction::Left));

        transition_table.insert((3, Direction::Left), (2, Direction::Right));
        transition_table.insert((3, Direction::Down), (4, Direction::Up));
        transition_table.insert((3, Direction::Right), (5, Direction::Up));

        transition_table.insert((4, Direction::Left), (2, Direction::Down));

        transition_table.insert((5, Direction::Left), (4, Direction::Right));
    } else {
        transition_table.insert((0, Direction::Up), (5, Direction::Left));
        transition_table.insert((0, Direction::Left), (3, Direction::Left));
        transition_table.insert((0, Direction::Down), (2, Direction::Up));
        transition_table.insert((0, Direction::Right), (1, Direction::Left));

        transition_table.insert((1, Direction::Up), (5, Direction::Down));
        transition_table.insert((1, Direction::Down), (2, Direction::Right));
        transition_table.insert((1, Direction::Right), (4, Direction::Right));

        transition_table.insert((2, Direction::Left), (3, Direction::Up));
        transition_table.insert((2, Direction::Down), (4, Direction::Up));

        transition_table.insert((3, Direction::Right), (4, Direction::Left));

        transition_table.insert((5, Direction::Up), (3, Direction::Down));
        transition_table.insert((5, Direction::Right), (4, Direction::Down));
    }

    assert!(has_unique_elements(
        transition_table
            .left_values()
            .chain(transition_table.right_values())
    ));
    transition_table
}

fn invert_index(idx: usize, len: usize) -> usize {
    len - idx - 1
}

fn get_transition(transition: (usize, Direction), table: &Transitions) -> (usize, Direction) {
    let (board_number, direction) = if table.contains_left(&transition) {
        *table.get_by_left(&transition).unwrap()
    } else if table.contains_right(&transition) {
        *table.get_by_right(&transition).unwrap()
    } else {
        panic!("Transition {transition:?} not found");
    };
    // println!(
    //     "Transitioned from {transition:?} to {:?}",
    //     (board_number, direction)
    // );
    (board_number, direction)
}

fn global_pos(pos: Position, board_number: usize, starts: [(usize, usize); 6]) -> Position {
    pos + Position::new(starts[board_number].0, starts[board_number].1)
}

fn part2(input: &str, example: bool) -> usize {
    let (board, path) = input.split_once("\n\n").unwrap();
    let (boards, starts) = parse_cube_board(board);

    let transition_table = make_transition_table(example);

    for board in boards.iter() {
        println!("{}", pretty_print(board));
        println!();
    }

    let mut pos = Position::new(1, 1);
    let mut facing = Direction::Right;
    let mut board_number = 0;

    // visualize_print(&boards[board_number], pos, facing);

    let mut path = Path::new(path.trim());

    while let Some(action) = path.get_next() {
        match action {
            Action::Move(distance) => {
                for _ in 0..distance {
                    let next = pos + facing;
                    let value_at_next = boards[board_number].get(next.to_index()).unwrap();
                    match value_at_next {
                        '.' => pos = next,
                        '#' => break,
                        ' ' => {
                            if let Some((next_board_number, next_pos, next_facing)) =
                                wrap_position_cube(
                                    pos,
                                    facing,
                                    &boards,
                                    board_number,
                                    &transition_table,
                                )
                            {
                                pos = next_pos;
                                board_number = next_board_number;
                                facing = next_facing;
                            } else {
                                break;
                            }
                        }
                        _ => panic!("Unexpected board value {value_at_next}"),
                    }
                    // dbg!(board_number);
                    // visualize_print(&boards[board_number], pos, facing);
                }
            }
            Action::Turn(dir) => facing.turn(dir),
        }
        // dbg!(board_number);
        // visualize_print(&boards[board_number], pos, facing);
    }
    password(global_pos(pos, board_number, starts), facing)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(22)?;
    dbg!(part1(input));
    dbg!(part2(input, false));

    Ok(())
}

#[test]
fn example() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    assert_eq!(part1(input), 6032);
    assert_eq!(part2(input, true), 5031);
}

#[test]
fn task() {
    let input = &read_input_to_string(22).unwrap();
    assert_eq!(part1(input), 123046);
    assert_eq!(part2(input, false), 195032);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(22).unwrap();
        part1(input);
        part2(input, false);
    })
}

extern crate test;

use std::{
    cmp::{max, Ordering},
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

use itertools::enumerate;
use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "R" => Direction::Right,
            c => panic!("Unknown str for Direction {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to_index(self) -> [usize; 2] {
        [self.y.try_into().unwrap(), self.x.try_into().unwrap()]
    }

    fn to_unit(self) -> Self {
        Self {
            x: to_unit(self.x),
            y: to_unit(self.y),
        }
    }

    fn touching(self, other: Self) -> bool {
        let diff = self - other;
        max(diff.x.abs(), diff.y.abs()) <= 1
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

fn pretty_print(arr: &Array2<char>, head: [usize; 2], tail: [usize; 2], print_ht: bool) -> String {
    // TODO: Ditch the Array2 and just generate a nice Array2
    // for visualization from the HashMap
    let mut arr = arr.clone();
    if print_ht {
        arr[tail] = 'T';
        arr[head] = 'H'; // H last so we overwrite T if overlapping
    }
    let mut result = String::new();
    for row in arr.rows() {
        for elem in row {
            result.push(*elem);
        }
        result.push('\n');
    }

    result
}

fn to_unit(v: i32) -> i32 {
    match v.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

#[allow(dead_code)]
fn part1_visualize(input: &str) -> String {
    let mut visualization = Array2::<char>::from_elem((5, 6), '.');
    visualization[Position::new(0, 4).to_index()] = 's';
    let mut head = Position::new(0, 4);
    let mut tail = Position::new(0, 4);

    println!(
        "{}",
        pretty_print(&visualization, head.to_index(), tail.to_index(), true)
    );

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let dir: Direction = dir.into();
        for _ in 0..count.parse().unwrap() {
            head += dir;
            if !head.touching(tail) {
                tail += (head - tail).to_unit();
            }
            visualization[tail.to_index()] = '#';
        }
        println!(
            "{}",
            pretty_print(&visualization, head.to_index(), tail.to_index(), true)
        );
    }
    println!(
        "{}",
        pretty_print(&visualization, head.to_index(), tail.to_index(), true)
    );
    println!(
        "{}",
        pretty_print(&visualization, head.to_index(), tail.to_index(), false)
    );
    pretty_print(&visualization, head.to_index(), tail.to_index(), false)
}

type VisitMap = HashSet<Position>;

fn part1(input: &str) -> usize {
    let mut visualization: VisitMap = HashSet::new();
    let mut head = Position::new(0, 0);
    let mut tail = Position::new(0, 0);

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let dir: Direction = dir.into();
        for _ in 0..count.parse().unwrap() {
            head += dir;
            if !head.touching(tail) {
                tail += (head - tail).to_unit();
            }
            visualization.insert(tail);
        }
    }

    visualization.len()
}

fn part2(input: &str) -> usize {
    let mut visualization: VisitMap = HashSet::new();
    let mut tails: [Position; 10] = [Position::new(0, 0); 10];

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let dir: Direction = dir.into();
        for _ in 0..count.parse().unwrap() {
            tails[0] += dir;
            for (i, current_tail) in enumerate(tails).skip(1) {
                let forward_tail = tails[i - 1];
                if !forward_tail.touching(current_tail) {
                    tails[i] += (forward_tail - current_tail).to_unit();
                }
            }
            visualization.insert(tails[tails.len()-1]);
        }
    }

    visualization.len()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(9)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_visualize() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(
        part1_visualize(input),
        "..##..
...##.
.####.
....#.
####..
"
    );
}

#[test]
fn example() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 1);
}

#[test]
fn example2() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part2(input), 36);
}

#[test]
fn task() {
    let input = &read_input_to_string(9).unwrap();
    assert_eq!(part1(input), 5907);
    assert_eq!(part2(input), 2303);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(9).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

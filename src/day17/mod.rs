extern crate test;

use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::Itertools;
use ndarray::{Array2, Axis};
#[cfg(test)]
use test::Bencher;

use crate::utils::{pretty_print, read_input_to_string};

const WIDTH: usize = 7;

enum ShapeType {
    HorizontalLine,
    Cross,
    Angle,
    VerticalLine,
    Square,
}

struct Shape {
    positions: Vec<(usize, usize)>,
}

impl Shape {
    fn initial(kind: &ShapeType, height: usize) -> Self {
        let mut positions: Vec<_> = match kind {
            ShapeType::HorizontalLine => [(2, 0), (3, 0), (4, 0), (5, 0)].into(),
            ShapeType::Cross => [(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)].into(),
            ShapeType::Angle => [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)].into(),
            ShapeType::VerticalLine => [(2, 0), (2, 1), (2, 2), (2, 3)].into(),
            ShapeType::Square => [(2, 0), (3, 0), (2, 1), (3, 1)].into(),
        };
        positions.iter_mut().for_each(|(_, y)| *y += height);
        Shape { positions }
    }

    fn move_left(&mut self, occupied: &HashSet<(usize, usize)>) {
        if self.positions.iter().all(|(x, _)| x > &0) {
            let moved = self
                .positions
                .iter()
                .map(|(x, y)| (x - 1, *y))
                .collect_vec();
            if moved.iter().all(|(x, y)| !occupied.contains(&(*x, *y))) {
                self.positions = moved;
            }
        }
    }

    fn move_right(&mut self, occupied: &HashSet<(usize, usize)>) {
        if self.positions.iter().all(|(x, _)| x < &6) {
            let moved = self
                .positions
                .iter()
                .map(|(x, y)| (x + 1, *y))
                .collect_vec();
            if moved.iter().all(|(x, y)| !occupied.contains(&(*x, *y))) {
                self.positions = moved;
            }
        }
    }

    fn move_down(&mut self, occupied: &HashSet<(usize, usize)>) -> Option<()> {
        if self.positions.iter().all(|(_, y)| y > &0) {
            let moved = self
                .positions
                .iter()
                .map(|(x, y)| (*x, y - 1))
                .collect_vec();
            if moved.iter().all(|(x, y)| !occupied.contains(&(*x, *y))) {
                self.positions = moved;
                return Some(());
            }
        }
        None
    }
}

struct Tower {
    heights: [(usize, usize); WIDTH],
    occupied: HashSet<(usize, usize)>,
}

impl Tower {
    fn new() -> Self {
        let mut heights = [(0, 0); WIDTH];
        heights
            .iter_mut()
            .enumerate()
            .for_each(|(i, (x, _))| *x += i);
        let occupied = HashSet::from(heights);
        Tower { heights, occupied }
    }

    fn top(&self) -> usize {
        *self.heights.iter().map(|(_, y)| y).max().unwrap()
    }

    fn top_diffs(&self) -> [usize; WIDTH] {
        let mn = self.heights.iter().map(|(_, y)| y).min().unwrap();
        self.heights
            .iter()
            .map(|(_, y)| y - mn)
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn visualize(&self) -> String {
        let mut arr = Array2::<char>::from_elem((self.top() + 1, WIDTH), '.');
        self.occupied.iter().for_each(|(x, y)| arr[(*y, *x)] = '#');
        arr.invert_axis(Axis(0));

        pretty_print(&arr)
    }

    fn print_visualize(&self) {
        println!("{}", self.visualize());
    }
}

const SHAPES: [ShapeType; 5] = [
    ShapeType::HorizontalLine,
    ShapeType::Cross,
    ShapeType::Angle,
    ShapeType::VerticalLine,
    ShapeType::Square,
];

fn parts(input: &str, shapes_to_land: usize) -> usize {
    let mut seen = HashMap::new();
    let mut tower = Tower::new();
    let mut jets = input.trim().char_indices().cycle();
    let mut shapes = SHAPES.iter().enumerate().cycle();
    let mut next_shape = true;
    let mut shape = Shape::initial(&ShapeType::HorizontalLine, 3); // Placeholder
    let mut shape_count = 0;

    let mut shape_idx = 0;
    let mut jet_idx = 0;

    let mut extra_height = 0;

    // tower.print_visualize();
    // println!();

    let mut has_skipped = false;

    while shape_count < shapes_to_land {
        if next_shape {
            let next_shape_type = shapes.next().unwrap();
            shape_idx = next_shape_type.0;
            shape = Shape::initial(next_shape_type.1, tower.top() + 4);
            next_shape = false;

            if !has_skipped {
                let key = (shape_idx, jet_idx, tower.top_diffs());
                if let Entry::Vacant(e) = seen.entry(key) {
                    e.insert((shape_count, tower.top()));
                } else {
                    let (last_shape_count, last_height) = seen[&key];
                    let shapes_since = shape_count - last_shape_count;
                    let height_since = tower.top() - last_height;
                    let remaining_shape_count = shapes_to_land - shape_count;

                    let repeats = remaining_shape_count / shapes_since;
                    let remaining = remaining_shape_count % shapes_since;

                    extra_height = repeats * height_since;
                    shape_count += repeats * shapes_since;

                    dbg!(key);
                    dbg!(last_shape_count, last_height);
                    dbg!(shapes_since, height_since);
                    dbg!(remaining);

                    has_skipped = true;
                }
            }
        }
        let next_jets = jets.next().unwrap();
        jet_idx = next_jets.0;

        match next_jets.1 {
            '<' => shape.move_left(&tower.occupied),
            '>' => shape.move_right(&tower.occupied),
            _ => panic!("bad jet {}", next_jets.1),
        }
        if shape.move_down(&tower.occupied).is_none() {
            shape.positions.iter().for_each(|(x, y)| {
                tower.occupied.insert((*x, *y));
                if y > &tower.heights[*x].1 {
                    tower.heights[*x].1 = *y;
                }
            });
            shape_count += 1;
            next_shape = true;

            // tower.print_visualize();
            // println!();
        }
    }
    tower.top() + extra_height
}

fn part1(input: &str) -> usize {
    parts(input, 2022)
}

fn part2(input: &str) -> usize {
    parts(input, 1000000000000)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(17)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
    assert_eq!(part2(input), 1514285714288);
}

#[test]
fn task() {
    let input = &read_input_to_string(17).unwrap();
    assert_eq!(part1(input), 3219);
    // assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(17).unwrap();
        part1(input);
        part2(input);
    })
}

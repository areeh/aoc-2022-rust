extern crate test;

use std::{
    cmp::Ordering,
    isize,
    ops::{Add, Range, Sub},
};

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

const P1_LOC: usize = 2_000_000;
const P2_MAX: usize = 4_000_000;

fn parse_line(line: &str) -> (Point, Point) {
    let line = line.replace("x=", "").replace("y=", "");
    let (sensor, beacon) = line.split_once(": ").unwrap();
    (
        sensor.strip_prefix("Sensor at ").unwrap().into(),
        beacon.strip_prefix("closest beacon is at ").unwrap().into(),
    )
}

fn to_unit(v: isize) -> isize {
    match v.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn to_index(self) -> (usize, usize) {
        (self.0.try_into().unwrap(), self.1.try_into().unwrap())
    }

    fn to_unit(self) -> Self {
        Self(to_unit(self.0), to_unit(self.1))
    }

    fn manhattan(&self, other: &Point) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s
            .split_once(", ")
            .unwrap_or_else(|| panic!("Unexpected string for Point {s}"));
        Point(
            x.parse().unwrap_or_else(|_| panic!("bad digit {x}")),
            y.parse().unwrap_or_else(|_| panic!("bad digit {y}")),
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(
            self.0.checked_sub(other.0).unwrap(),
            self.1.checked_sub(other.1).unwrap(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    m: isize,
    b: isize,
    min_x: isize,
    max_x: isize,
}

impl Line {
    fn is_on_side_inclusive(&self, point: Point, above: bool) -> bool {
        if above {
            point.1 <= self.m * point.0 + self.b
        } else {
            point.1 >= self.m * point.0 + self.b
        }
    }

    fn from_start_end(start: Point, end: Point) -> Self {
        let m = (end - start).to_unit().1;
        if m == 0 {
            panic!("got bad m for start {start:?} end {end:?}");
        }
        Line {
            m,
            b: start.1 - (m * start.0),
            min_x: start.0.min(end.0),
            max_x: start.0.max(end.0),
        }
    }

    fn y_at(&self, x: isize) -> Point {
        Point(x, self.m * x + self.b)
    }

    fn x_at(&self, y: isize) -> Point {
        Point((y - self.b) / self.m, y)
    }

    fn is_on(&self, point: Point) -> bool {
        if self.m == 0 {
            return point.1 == self.b;
        }
        point.0 == self.x_at(point.1).0
    }

    fn walk_in_range(self, xy_range: Range<isize>) -> impl Iterator<Item = Point> {
        let valid_range = if self.m < 0 {
            *[self.x_at(xy_range.end - 1).0, xy_range.start, self.min_x]
                .iter()
                .max()
                .unwrap()
                ..*[xy_range.end, self.max_x, self.x_at(xy_range.start).0]
                    .iter()
                    .min()
                    .unwrap()
        } else {
            *[self.x_at(xy_range.start).0, xy_range.start, self.min_x]
                .iter()
                .max()
                .unwrap()
                ..*[self.x_at(xy_range.end - 1).0, xy_range.end, self.max_x]
                    .iter()
                    .min()
                    .unwrap()
        };

        valid_range.into_iter().map(move |x| self.y_at(x))
    }

    fn add_b(&self, b: isize) -> Line {
        Line {
            m: self.m,
            b: self.b + b,
            min_x: self.min_x,
            max_x: self.max_x,
        }
    }

    fn intersect(&self, other: &Line) -> Option<Point> {
        if self.m == other.m {
            return None;
        }

        let y = (self.m * ((other.b - self.b) / (self.m - other.m))) + self.b;
        Some(if self.m != 0 {
            self.x_at(y)
        } else {
            other.x_at(y)
        })
    }
}

#[derive(Debug)]
struct Diamond {
    top_right: Line,
    left_top: Line,
    left_bottom: Line,
    bottom_right: Line,
}

impl Diamond {
    fn from_center_radius(center: Point, radius: usize) -> Self {
        if radius == 0 {
            return Self {
                top_right: Line {
                    m: 1,
                    b: center.1,
                    min_x: center.0,
                    max_x: center.0,
                },
                left_top: Line {
                    m: 1,
                    b: center.1,
                    min_x: center.0,
                    max_x: center.0,
                },
                left_bottom: Line {
                    m: 1,
                    b: center.1,
                    min_x: center.0,
                    max_x: center.0,
                },
                bottom_right: Line {
                    m: 1,
                    b: center.1,
                    min_x: center.0,
                    max_x: center.0,
                },
            };
        }

        let top = center - Point(0, radius as isize);
        let left = center - Point(radius as isize, 0);
        let bottom = center + Point(0, radius as isize);
        let right = center + Point(radius as isize, 0);

        Self {
            left_top: Line::from_start_end(left, top),
            top_right: Line::from_start_end(top, right),
            left_bottom: Line::from_start_end(left, bottom),
            bottom_right: Line::from_start_end(bottom, right),
        }
    }

    fn is_inside(&self, point: Point) -> bool {
        self.top_right.is_on_side_inclusive(point, false)
            && self.left_top.is_on_side_inclusive(point, false)
            && self.left_bottom.is_on_side_inclusive(point, true)
            && self.bottom_right.is_on_side_inclusive(point, true)
    }

    fn walk_outer(self) -> impl Iterator<Item = Line> {
        [
            self.top_right,
            self.left_top,
            self.left_bottom,
            self.bottom_right,
        ]
        .into_iter()
    }

    fn walk_corners(center: Point, radius: usize) -> impl Iterator<Item = Point> {
        [
            center - Point(0, radius as isize),
            center - Point(radius as isize, 0),
            center + Point(0, radius as isize),
            center + Point(radius as isize, 0),
        ]
        .into_iter()
    }
}

fn range_at_distance(x: isize, radius: usize, distance: usize) -> Option<(isize, isize)> {
    if distance > radius {
        return None;
    }

    let delta = radius.checked_sub(distance).unwrap();
    Some((x - delta as isize, x + delta as isize))
}

fn merge_ranges(mut ranges: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ranges
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<(isize, isize)>, range| {
            if let Some(last_merged) = acc.last_mut() {
                if last_merged.1 >= (range.0 - 1) {
                    last_merged.1 = range.1.max(last_merged.1);
                } else {
                    acc.push(range);
                }
            } else {
                acc.push(range);
            }
            acc
        })
}

fn count_points_in_ranges(ranges: Vec<(isize, isize)>) -> isize {
    ranges.iter().map(|(l, r)| *r - *l).sum()
}

fn part1(input: &str, y: usize) -> usize {
    let mut ranges: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .filter_map(|(sensor, radius)| {
            range_at_distance(sensor.0, radius, (sensor.1 - y as isize).unsigned_abs())
        })
        .collect();
    ranges = merge_ranges(ranges);

    count_points_in_ranges(ranges) as usize
}

fn part2(input: &str, mx: usize) -> usize {
    let diamonds: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .map(|(sensor, radius)| Diamond::from_center_radius(sensor, radius))
        .collect();

    let outer_lines: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .map(|(sensor, radius)| Diamond::from_center_radius(sensor, radius + 1))
        .flat_map(|diamond| diamond.walk_outer())
        .collect();

    let check_range = 0..(mx + 1) as isize;

    for pt in outer_lines.iter().permutations(2).filter_map(|v| {
        if let Some(pt) = v[0].intersect(v[1]) {
            if check_range.contains(&pt.0) && check_range.contains(&pt.1) {
                return Some(pt);
            }
        }
        None
    }) {
        if diamonds.iter().all(|diamond| !diamond.is_inside(pt)) {
            return pt.0 as usize * P2_MAX + pt.1 as usize;
        }
    }

    panic!("No solution found")
}

#[allow(dead_code)]
fn part2_2(input: &str, mx: usize) -> usize {
    let sensor_radiuses: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .collect_vec();

    for y in 0..mx as isize {
        let mut ranges = sensor_radiuses
            .iter()
            .filter_map(|(sensor, radius)| {
                range_at_distance(sensor.0, *radius, (sensor.1 - y).unsigned_abs())
            })
            .collect();

        ranges = merge_ranges(ranges);
        if ranges.len() > 1 {
            dbg!(ranges.clone(), y);
            return (ranges[0].1 + 1) as usize * P2_MAX + y as usize;
        }
    }
    panic!("No solution found");
}

#[allow(dead_code)]
fn part2_3(input: &str, mx: usize) -> usize {
    let diamonds: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .map(|(sensor, radius)| Diamond::from_center_radius(sensor, radius))
        .collect();

    let diamond_outers: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(sensor, beacon)| (sensor, sensor.manhattan(&beacon)))
        .map(|(sensor, radius)| Diamond::from_center_radius(sensor, radius + 1))
        .collect();

    for pt in diamond_outers.into_iter().flat_map(|diamond| {
        diamond
            .walk_outer()
            .flat_map(|line| line.walk_in_range(0..mx as isize + 1))
    }) {
        if diamonds.iter().all(|diamond| !diamond.is_inside(pt)) {
            dbg!(pt);
            return pt.0 as usize * P2_MAX + pt.1 as usize;
        }
    }
    panic!("No solution found")
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(15)?;
    dbg!(part1(input, P1_LOC));
    dbg!(part2(input, P2_MAX));

    Ok(())
}

#[test]
fn example() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    assert_eq!(part1(input, 10), 26);
    assert_eq!(part2(input, 20), 56000011);
    assert_eq!(part2_2(input, 20), 56000011);
    assert_eq!(part2(input, 20), 56000011);
}

#[test]
fn task() {
    let input = &read_input_to_string(15).unwrap();
    assert_eq!(part1(input, P1_LOC), 4919281);
    // assert_eq!(part2(input, P2_MAX), 12630143363767);
    // assert_eq!(part2_2(input, P2_MAX), 12630143363767);
    assert_eq!(part2(input, P2_MAX), 12630143363767);
}

// #[bench]
// fn task_bench(b: &mut Bencher) {
//     let input = &read_input_to_string(15).unwrap();
//     b.iter(|| {
//         part2_3(input, P2_MAX);
//     })
// }

// #[bench]
// fn task_bench_2(b: &mut Bencher) {
//     let input = &read_input_to_string(15).unwrap();
//     b.iter(|| {
//         part2_2(input, P2_MAX);
//     })
// }

#[bench]
fn task_bench_3(b: &mut Bencher) {
    let input = &read_input_to_string(15).unwrap();
    b.iter(|| {
        part1(input, P1_LOC);
        part2(input, P2_MAX);
    })
}

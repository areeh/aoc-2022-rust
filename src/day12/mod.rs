extern crate test;

use std::collections::HashSet;

use itertools::Itertools;
use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_input(input: &str) -> Array2<char> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line.trim().chars().collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;

    Array2::from_shape_vec((n_rows, board_width), data).unwrap()
}

const CARDINALS: &[(usize, usize); 4] = &[(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];

fn get_neighbors(
    (x, y): (usize, usize),
    floor: &Array2<char>,
    down: bool,
) -> Vec<((usize, usize), char)> {
    let mut neighbors = Vec::new();
    for dir in CARDINALS {
        let next = (x.wrapping_add(dir.0), y.wrapping_add(dir.1));
        if let Some(v) = floor.get(next) {
            let mut height = (*v as i8) - (floor[[x, y]] as i8);
            if down {
                height *= -1;
            }

            if height <= 1 {
                neighbors.push((next, *v));
            }
        }
    }
    neighbors
}

fn loop_time(arr: &Array2<char>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut viz = arr.clone();

    let mut start_visited = HashSet::new();
    let mut start_heads = vec![start];

    let mut end_visited = HashSet::new();
    let mut end_heads = vec![end];

    let mut cnt = 0;
    loop {
        cnt += 1;
        let mut next_start_heads = Vec::new();
        for head in start_heads {
            for (i, _) in get_neighbors(head, arr, false) {
                if end_visited.contains(&i) {
                    return Some(cnt);
                }

                if !start_visited.contains(&i) {
                    viz[i] = 'H';
                    next_start_heads.push(i);
                    start_visited.insert(i);
                }
            }
        }
        start_heads = next_start_heads;

        cnt += 1;
        let mut next_end_heads = Vec::new();
        for head in end_heads {
            for (i, _) in get_neighbors(head, arr, true) {
                if start_visited.contains(&i) {
                    return Some(cnt);
                }

                if !end_visited.contains(&i) {
                    viz[i] = 'T';
                    next_end_heads.push(i);
                    end_visited.insert(i);
                }
            }
        }
        end_heads = next_end_heads;

        if start_heads.is_empty() && end_heads.is_empty() {
            return None;
        }
    }
}

fn part1(input: &str) -> usize {
    let mut arr = parse_input(input);
    let (start, _) = arr.indexed_iter().find(|(_, c)| **c == 'S').unwrap();
    let (end, _) = arr.indexed_iter().find(|(_, c)| **c == 'E').unwrap();
    arr[start] = 'a';
    arr[end] = 'z';

    if let Some(path_length) = loop_time(&arr, start, end) {
        path_length
    } else {
        panic!("Failure");
    }
}

fn part2(input: &str) -> usize {
    let mut arr = parse_input(input);
    let (start, _) = arr.indexed_iter().find(|(_, c)| **c == 'S').unwrap();
    let (end, _) = arr.indexed_iter().find(|(_, c)| **c == 'E').unwrap();
    arr[start] = 'a';
    arr[end] = 'z';

    let starts: Vec<_> = arr
        .indexed_iter()
        .filter_map(|(i, c)| if *c == 'a' { Some(i) } else { None })
        .collect();

    let mut mn = arr.len();
    for start in starts {
        let maybe_path_length = loop_time(&arr, start, end);
        if let Some(path_length) = maybe_path_length {
            if path_length < mn {
                mn = path_length;
            }
        }
    }
    mn
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(12)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(part1(input), 31);
    assert_eq!(part2(input), 29);
}

#[test]
fn task() {
    let input = &read_input_to_string(12).unwrap();
    assert_eq!(part1(input), 352);
    assert_eq!(part2(input), 345);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(12).unwrap();
        part1(input);
        part2(input);
    })
}

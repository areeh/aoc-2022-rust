extern crate test;

use std::ops::Range;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

const GROVE_COORDS: [usize; 3] = [1000, 2000, 3000];

fn euclidian_mod(v: i64, len: usize) -> usize {
    let len = len as i64;
    if v < 0 {
        (((v % len) + len) % len) as usize
    } else {
        (v % len) as usize
    }
}

fn mixing(indices: &mut Vec<usize>, number_file: &[i64], indices_range: &Range<usize>) {
    // let mut mixing = number_file.to_vec();
    // dbg!(&mixing);
    for start_idx in indices_range.clone() {
        let old_position = indices.iter().position(|idx| *idx == start_idx).unwrap();
        indices.remove(old_position);
        indices.insert(
            euclidian_mod(old_position as i64 + number_file[start_idx], indices.len()),
            start_idx,
        );

        // for (current_idx, start_idx) in indices.iter().enumerate() {
        //     mixing[current_idx] = number_file[*start_idx];
        // }
        // dbg!(&mixing);
    }
}

fn part1(input: &str) -> i64 {
    let number_file = parse_input(input);
    let indices_range = 0..number_file.len();

    // Value is start index, position is current index
    let mut indices: Vec<_> = indices_range.clone().collect();

    mixing(&mut indices, &number_file, &indices_range);

    let start_idx_of_zero = number_file.iter().position(|v| *v == 0).unwrap();
    let idx_of_zero = indices
        .iter()
        .position(|idx| *idx == start_idx_of_zero)
        .unwrap();

    GROVE_COORDS
        .iter()
        .map(|offset| number_file[indices[(idx_of_zero + offset) % indices.len()]])
        .sum()
}

fn part2(input: &str) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let mut number_file = parse_input(input);
    number_file.iter_mut().for_each(|v| *v *= DECRYPTION_KEY);

    let indices_range = 0..number_file.len();

    // Value is start index, position is current index
    let mut indices: Vec<_> = indices_range.clone().collect();

    for _ in 0..10 {
        mixing(&mut indices, &number_file, &indices_range);
    }

    let start_idx_of_zero = number_file.iter().position(|v| *v == 0).unwrap();
    let idx_of_zero = indices
        .iter()
        .position(|idx| *idx == start_idx_of_zero)
        .unwrap();

    GROVE_COORDS
        .iter()
        .map(|offset| number_file[indices[(idx_of_zero + offset) % indices.len()]])
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(20)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "1
2
-3
3
-2
0
4";
    assert_eq!(part1(input), 3);
    assert_eq!(part2(input), 1623178306);
}

#[test]
fn task() {
    let _input = &read_input_to_string(20).unwrap();
    // assert_eq!(part1(input), ());
    // assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let _input = &read_input_to_string(20).unwrap();
        // part1(input);
        // part2(input);
    })
}

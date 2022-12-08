extern crate test;

use itertools::Itertools;
use ndarray::{s, Array2, ArrayView1, Axis, Zip};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_input(input: &str) -> Array2<u8> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .unwrap_or_else(|| panic!("unknown digit {c}")) as u8
            })
            .collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;
    Array2::from_shape_vec((n_rows, board_width), data).unwrap()
}

fn mark_perimeter(visible: &mut Array2<u8>) {
    visible.row_mut(0).fill(1);
    visible.row_mut(visible.len_of(Axis(0)) - 1).fill(1);
    visible.column_mut(0).fill(1);
    visible.column_mut(visible.len_of(Axis(1)) - 1).fill(1);
}

fn count_visible(visible: &Array2<u8>) -> usize {
    visible.iter().filter(|v| **v > 0).count()
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let mut visible = Array2::<u8>::zeros(map.raw_dim());
    mark_perimeter(&mut visible);

    let mut curr_max = 0;
    for step in [-1, 1].iter() {
        for i in 1..map.len_of(Axis(0)) {
            Zip::from(visible.slice_mut(s![i, ..;*step]))
                .and(map.slice(s![i, ..;*step]))
                .for_each(|v, &t| {
                    if t as i32 > curr_max {
                        curr_max = t as i32;
                        *v += 1;
                    }
                });
            curr_max = 0;
        }
        for i in 1..map.len_of(Axis(1)) {
            Zip::from(visible.slice_mut(s![..;*step, i]))
                .and(map.slice(s![..;*step, i]))
                .for_each(|v, &t| {
                    if t as i32 > curr_max {
                        curr_max = t as i32;
                        *v += 1;
                    }
                });
            curr_max = 0;
        }
    }

    count_visible(&visible)
}

fn score_line(i: usize, line: ArrayView1<u8>, scenic_scores: &mut Array2<u32>, row: bool) {
    for (j, v) in line.indexed_iter() {
        if j == 0 || j == line.len() - 1 {
            continue;
        }
        let mut score = 0;
        for t in line.slice(s![0..j;-1]) {
            if t >= v {
                score += 1;
                break;
            } else {
                score += 1;
            }
        }

        if row {
            scenic_scores[[i, j]] *= score;
        } else {
            scenic_scores[[j, i]] *= score;
        }
        score = 0;

        for t in line.slice(s![j + 1..line.len()]) {
            if t >= v {
                score += 1;
                break;
            } else {
                score += 1;
            }
        }
        if row {
            scenic_scores[[i, j]] *= score;
        } else {
            scenic_scores[[j, i]] *= score;
        }
    }
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let mut scenic_scores = Array2::<u32>::ones(map.raw_dim());

    for row_idx in 1..map.len_of(Axis(1)) - 1 {
        let row = map.row(row_idx);
        score_line(row_idx, row, &mut scenic_scores, true);
    }

    for col_idx in 1..map.len_of(Axis(0)) - 1 {
        let col = map.column(col_idx);
        score_line(col_idx, col, &mut scenic_scores, false);
    }

    *scenic_scores.iter().max().unwrap() as usize
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(8)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_count_perimeter() {
    let mut arr = Array2::<u8>::zeros((5, 5));
    mark_perimeter(&mut arr);
    assert_eq!(arr.iter().filter(|v| **v > 0).count(), 16);
}

#[test]
fn example() {
    let input = "30373
25512
65332
33549
35390";
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 8);
}

#[test]
fn task() {
    let input = &read_input_to_string(8).unwrap();
    assert_eq!(part1(input), 1676);
    assert_eq!(part2(input), 313200);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(8).unwrap();
        part1(input);
        part2(input);
    })
}

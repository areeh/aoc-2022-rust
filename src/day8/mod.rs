extern crate test;

use itertools::Itertools;
use ndarray::{s, Array2, ArrayBase, Axis, FoldWhile, Ix2, RawData, Zip};
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

/// See: https://github.com/rust-ndarray/ndarray/issues/866
fn rot90<S>(arr: &mut ArrayBase<S, Ix2>)
where
    S: RawData,
{
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(0));
}

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    let mut visible = Array2::<u8>::zeros(map.raw_dim());
    mark_perimeter(&mut visible);

    let mut curr_max = 0;

    for _ in 0..4 {
        rot90(&mut map);
        rot90(&mut visible);

        for i in 1..map.len_of(Axis(0)) {
            Zip::from(visible.slice_mut(s![i, ..]))
                .and(map.slice(s![i, ..]))
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

fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let mut scenic_scores = Array2::<u32>::ones(map.raw_dim());
    let side_len = map.len_of(Axis(0));

    for _ in 0..2 {
        rot90(&mut map);
        rot90(&mut scenic_scores);

        Zip::from(scenic_scores.rows_mut())
            .and(map.rows())
            .for_each(|mut score_row, tree_row| {
                let mut score_row = score_row.slice_mut(s![1..side_len - 1]);
                for (i, v) in tree_row.slice(s![1..side_len - 1]).indexed_iter() {
                    let score =
                        Zip::from(tree_row.slice(s![..i+1;-1])).fold_while(0u32, |acc, t| {
                            if t >= v {
                                FoldWhile::Done(acc + 1)
                            } else {
                                FoldWhile::Continue(acc + 1)
                            }
                        });
                    score_row[i] *= score.into_inner();

                    // Do the right side of the base too to save some looping
                    let score =
                        Zip::from(tree_row.slice(s![i + 2..])).fold_while(0u32, |acc, t| {
                            if t >= v {
                                FoldWhile::Done(acc + 1)
                            } else {
                                FoldWhile::Continue(acc + 1)
                            }
                        });
                    score_row[i] *= score.into_inner();
                }
            });
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
    let input = &read_input_to_string(8).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

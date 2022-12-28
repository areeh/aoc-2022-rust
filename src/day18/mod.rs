extern crate test;

use itertools::Itertools;
use ndarray::Array3;
use ndarray_ndimage::{convolve, pad, BorderMode, PadMode};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

type Map = Array3<u8>;

const STAR: [[usize; 3]; 6] = [
    [1, 0, 0],
    [usize::MAX, 0, 0],
    [0, 1, 0],
    [0, usize::MAX, 0],
    [0, 0, 1],
    [0, 0, usize::MAX],
];

fn parse_input(input: &str) -> Map {
    let mut arr = Array3::zeros((20, 20, 20));

    input.lines().for_each(|line| {
        let idx: [usize; 3] = line
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        arr[idx] = 1;
    });

    arr
}

fn neighbor_mask() -> Map {
    let mut arr = Array3::zeros((3, 3, 3));
    arr[[1, 0, 1]] = 1;

    arr[[0, 1, 1]] = 1;
    arr[[2, 1, 1]] = 1;
    arr[[1, 1, 0]] = 1;
    arr[[1, 1, 2]] = 1;

    arr[[1, 2, 1]] = 1;
    arr
}

fn neighbors(map: &Map, mask: &Map) -> Map {
    convolve(map, mask, BorderMode::Constant(0), 0)
}

fn part1(input: &str) -> usize {
    let arr = parse_input(input);
    let neighbors = neighbors(&arr, &neighbor_mask());
    neighbors
        .iter()
        .zip(arr.iter())
        .map(|(neighbor, droplet)| {
            if droplet == &1 {
                (6 - neighbor) as usize
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn flood_fill(map: &Map, start: [usize; 3]) -> Map {
    let mut map = map.clone();
    let mut to_visit = Vec::from([start]);

    while let Some(next) = to_visit.pop() {
        if let Some(v) = map.get_mut(next) {
            if *v != 0 {
                continue;
            }

            *v = 2;

            for dir in STAR {
                to_visit.push([
                    next[0].wrapping_add(dir[0]),
                    next[1].wrapping_add(dir[1]),
                    next[2].wrapping_add(dir[2]),
                ]);
            }
        }
    }
    map
}

fn part2(input: &str) -> usize {
    let arr = parse_input(input);
    let arr = pad(&arr, &[[1, 1], [1, 1], [1, 1]], PadMode::Constant(0));
    let mut exposed_air = flood_fill(&arr, [0, 0, 0]);
    exposed_air.mapv_inplace(|v| if v == 0 { 1 } else { v });
    exposed_air.mapv_inplace(|v| if v == 2 { 0 } else { v });

    let neighbors = neighbors(&exposed_air, &neighbor_mask());

    neighbors
        .iter()
        .zip(arr.iter())
        .map(|(neighbor, droplet)| {
            if droplet == &1 {
                (6 - neighbor) as usize
            } else {
                0
            }
        })
        .sum::<usize>()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(18)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_eq!(part1(input), 64);
    assert_eq!(part2(input), 58);
}

#[test]
fn task() {
    let input = &read_input_to_string(18).unwrap();
    assert_eq!(part1(input), 3326);
    assert_eq!(part2(input), 1996);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(18).unwrap();
        part1(input);
        part2(input);
    })
}

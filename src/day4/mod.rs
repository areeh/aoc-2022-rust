extern crate test;
use anyhow::{anyhow, Context, Ok, Result};
use std::fs;

use itertools::{process_results, Itertools};

#[cfg(test)]
use test::Bencher;

fn input1() -> Result<String> {
    fs::read_to_string("./src/day4/input.txt").context("Failed to read input")
}

fn parse_assignment(pair: &str) -> Result<((u32, u32), (u32, u32))> {
    let out = pair.split(',').map(|p| {
        let (a, b) = p
            .split('-')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Could not collect inner tuple"))?;
        Ok((a.parse()?, b.parse()?))
    });
    let out = process_results(out, |iter| {
        iter.collect_tuple()
            .ok_or_else(|| anyhow!("Could not collect outer tuple"))
    })??;
    Ok(out)
}

fn fully_contains(pair: ((u32, u32), (u32, u32))) -> bool {
    let ((al, ar), (bl, br)) = pair;
    (al == bl) || (ar == br) || ((al < bl) == (ar > br))
}

fn part1(assignments: String) -> Result<u32> {
    assignments
        .lines()
        .map(|line| parse_assignment(line).map(|pair| fully_contains(pair) as u32))
        .sum()
}

fn overlaps(pair: ((u32, u32), (u32, u32))) -> bool {
    let ((al, ar), (bl, br)) = pair;
    // Check if the end of the leftmost segment overlaps the start of the rightmost segment
    // We already did this last year!?!?!??!
    ar.min(br) >= al.max(bl)
}

fn part2(assignments: String) -> Result<u32> {
    assignments
        .lines()
        .map(|line| parse_assignment(line).map(|pair| overlaps(pair) as u32))
        .sum()
}

pub fn main() -> Result<()> {
    let input = input1()?;
    dbg!(part1(input.clone())?);
    dbg!(part2(input)?);

    Ok(())
}

#[test]
fn example() -> Result<()> {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        .to_owned();
    assert_eq!(part1(input.clone())?, 2);
    assert_eq!(part2(input)?, 4);
    Ok(())
}

#[test]
fn task() -> Result<()> {
    let input = input1().unwrap();
    assert_eq!(part1(input.clone())?, 584);
    assert_eq!(part2(input)?, 933);
    Ok(())
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = input1().unwrap();
        part1(input.clone())?;
        part2(input)?;
        Ok(())
    })
}

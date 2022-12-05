extern crate test;
use anyhow::{anyhow, Ok, Result};

use itertools::{process_results, Itertools};

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

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

fn fully_contains(((al, ar), (bl, br)): ((u32, u32), (u32, u32))) -> bool {
    (al == bl) || (ar == br) || ((al < bl) == (ar > br))
}

fn part1(assignments: &str) -> Result<u32> {
    assignments
        .lines()
        .map(|line| parse_assignment(line).map(|pair| fully_contains(pair) as u32))
        .sum()
}

fn overlaps(((al, ar), (bl, br)): ((u32, u32), (u32, u32))) -> bool {
    // Check if the end of the leftmost segment overlaps the start of the rightmost segment
    // We already did this last year!?!?!??!
    ar.min(br) >= al.max(bl)
}

fn part2(assignments: &str) -> Result<u32> {
    assignments
        .lines()
        .map(|line| parse_assignment(line).map(|pair| overlaps(pair) as u32))
        .sum()
}

pub fn main() -> Result<()> {
    let input = &read_input_to_string(4)?;
    dbg!(part1(input)?);
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
2-6,4-8";
    assert_eq!(part1(input)?, 2);
    assert_eq!(part2(input)?, 4);
    Ok(())
}

#[test]
fn task() -> Result<()> {
    let input = &read_input_to_string(4).unwrap();
    assert_eq!(part1(input)?, 584);
    assert_eq!(part2(input)?, 933);
    Ok(())
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(4).unwrap();
        part1(input)?;
        part2(input)?;
        Ok(())
    })
}

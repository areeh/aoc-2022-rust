use std::fs;

use itertools::Itertools;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day3/input.txt")
}

fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96u32
    } else {
        c as u32 - 64u32 + 26u32
    }
}

fn char_in_both_halves(line: &str) -> char {
    let lhs = &line[0..line.len() / 2];
    line[line.len() / 2..line.len()]
        .chars()
        .find(|c| lhs.contains(*c))
        .unwrap()
}

fn part1(contents: String) -> u32 {
    contents
        .lines()
        .map(|line| char_to_priority(char_in_both_halves(line)))
        .sum()
}

fn common_char((one, two, three): (&str, &str, &str)) -> char {
    one.chars()
        .filter(|c| two.contains(*c))
        .find(|c| three.contains(*c))
        .unwrap()
}

fn part2(contents: String) -> u32 {
    contents
        .lines()
        .tuples()
        .map(|w| char_to_priority(common_char(w)))
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = input1()?;
    dbg!(part1(input.clone()));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .to_owned();
    assert_eq!(part1(input.clone()), 157);
    assert_eq!(part2(input), 70);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(input.clone()), 7990);
    assert_eq!(part2(input), 2602);
}

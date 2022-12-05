use itertools::Itertools;

use crate::utils::read_input_to_string;

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

fn part1(contents: &str) -> u32 {
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

fn part2(contents: &str) -> u32 {
    contents
        .lines()
        .tuples()
        .map(|w| char_to_priority(common_char(w)))
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(3)?;
    dbg!(part1(input));
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
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(input), 157);
    assert_eq!(part2(input), 70);
}

#[test]
fn task() {
    let input = &read_input_to_string(3).unwrap();
    assert_eq!(part1(input), 7990);
    assert_eq!(part2(input), 2602);
}

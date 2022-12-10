extern crate test;

use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_stacks<'a>(
    input_lines: &mut (impl Iterator<Item = &'a str> + std::clone::Clone),
) -> Vec<VecDeque<char>> {
    let n_stacks = (input_lines.clone().next().unwrap().len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); n_stacks];
    input_lines
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let mut char_iter = line.chars();
            char_iter.advance_by(1).unwrap();
            char_iter.step_by(4).enumerate().for_each(|(idx, c)| {
                if c != ' ' {
                    stacks[idx].push_front(c)
                }
            });
        });
    stacks
}

fn get_integers(text: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    RE.captures_iter(text)
        .map(|m| m[0].parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn do_one_move(mv: Vec<usize>, stacks: &mut [VecDeque<char>], rev: bool) {
    let from_stack = &mut stacks[mv[1] - 1];
    let crates = from_stack.split_off(from_stack.len() - mv[0]);
    if rev {
        crates
            .into_iter()
            .rev()
            .for_each(|v| stacks[mv[2] - 1].push_back(v))
    } else {
        crates
            .into_iter()
            .for_each(|v| stacks[mv[2] - 1].push_back(v))
    }
}

fn parts(input: &str, p2: bool) -> String {
    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);
    lines
        .map(get_integers)
        .for_each(|mv| do_one_move(mv, &mut stacks, !p2));
    stacks
        .into_iter()
        .map(|mut v| v.pop_back().unwrap())
        .collect()
}

fn part1(input: &str) -> String {
    parts(input, false)
}

fn part2(input: &str) -> String {
    parts(input, true)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(5)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_parse() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    ";
    let start = parse_stacks(&mut input.lines());
    assert_eq!(start, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
}

#[test]
fn example() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    assert_eq!(part1(input), "CMZ");
    assert_eq!(part2(input), "MCD");
}

#[test]
fn task() {
    let input = &read_input_to_string(5).unwrap();
    assert_eq!(part1(input), "LBLVVTVLP");
    assert_eq!(part2(input), "TPFFBDRJD");
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(5).unwrap();
        part1(input);
        part2(input);
    })
}

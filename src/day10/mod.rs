extern crate test;

use itertools_num::ItertoolsNum;
use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_line(line: &str) -> Instruction {
    match line.split_whitespace().collect::<Vec<_>>()[..] {
        ["noop"] => Instruction::Noop,
        ["addx", number] => Instruction::AddX(number.parse().unwrap()),
        _ => panic!("Unknown instruction {line}"),
    }
}

fn program(input: &str) -> Vec<i32> {
    let mut ret = vec![1, 0];
    for instruction in input.lines().map(parse_line) {
        match instruction {
            Instruction::Noop => ret.push(0),
            Instruction::AddX(value) => {
                ret.push(0); // addx starts with one idle cycle
                ret.push(value);
            }
        }
    }
    ret.iter().cumsum().collect()
}

fn part1(input: &str) -> i32 {
    let output = program(input);
    output
        .into_iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(i, v)| i as i32 * v)
        .sum()
}

fn pretty_print(arr: &Array2<char>) -> String {
    let mut result = String::new();
    for row in arr.rows() {
        for elem in row {
            result.push(*elem);
        }
        result.push('\n');
    }

    result.trim().to_owned()
}

fn position(cycle: usize) -> (usize, usize) {
    const SCREEN_SIZE: usize = 40;
    let i = cycle - 1; // 1-index to 0-index
    (i / SCREEN_SIZE, i % SCREEN_SIZE)
}

fn sprite_at_draw_position(x: i32, col: usize) -> bool {
    (x - col as i32).abs() <= 1
}

fn mark_if_at_draw_position(x: i32, cycle: usize, screen: &mut Array2<char>) {
    let (row, col) = position(cycle);
    if sprite_at_draw_position(x, col) {
        screen[[row, col]] = '#';
    }
}

fn part2(input: &str) -> String {
    let mut screen: Array2<char> = Array2::<char>::from_elem((6, 40), '.');
    let output = program(input);

    for (i, v) in output.into_iter().enumerate().skip(1) {
        mark_if_at_draw_position(v, i, &mut screen);
    }
    // println!("{}", pretty_print(&screen));
    pretty_print(&screen)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(10)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn tiny_example() {
    let input = "noop
addx 3
addx -5";
    assert_eq!(program(input), vec![1, 1, 1, 1, 4, 4, -1]);
}

#[test]
fn example() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part1(input), 13140);
    assert_eq!(
        part2(input),
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .trim()
    );
}

#[test]
fn task() {
    let input = &read_input_to_string(10).unwrap();
    assert_eq!(part1(input), 12740);
    assert_eq!(
        part2(input),
        "
###..###..###...##..###...##...##..####.
#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....
#..#.###..#..#.#..#.#..#.#..#.#....###..
###..#..#.###..####.###..####.#.##.#....
#.#..#..#.#....#..#.#.#..#..#.#..#.#....
#..#.###..#....#..#.#..#.#..#..###.#....
"
        .trim()
    );
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(10).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

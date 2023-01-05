extern crate test;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn snafu_char_to_num(c: &char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Unknown snafu char {c}"),
    }
}

const SNAFU_CHARS: [char; 5] = ['=', '-', '0', '1', '2'];

fn snafu_to_decimal(snafu: &str) -> i64 {
    const RADIX: i64 = 5;
    snafu
        .chars()
        .fold((RADIX.pow((snafu.len() - 1) as u32), 0), |(mul, acc), c| {
            (mul / 5, acc + snafu_char_to_num(&c) * mul)
        })
        .1
}

fn closest_snafu(num: i64, mul: i64) -> char {
    SNAFU_CHARS
        .into_iter()
        .min_by(|a, b| {
            num.abs_diff(snafu_char_to_num(a) * mul)
                .cmp(&num.abs_diff(snafu_char_to_num(b) * mul))
        })
        .unwrap()
}

fn decimal_to_snafu(decimal: usize) -> String {
    let mut n_digits = 0;
    while 2 * 5usize.pow(n_digits) < decimal {
        n_digits += 1;
    }
    dbg!(n_digits);

    let mut acc = 0;
    let mut ret = String::new();
    for place in (0..=n_digits).rev() {
        let mul = 5i64.pow(place);
        let closest = closest_snafu(decimal as i64 - acc, mul);
        ret.push(closest);
        acc += snafu_char_to_num(&closest) * mul;
    }
    ret
}

fn part1(input: &str) -> String {
    let decimal_sum: i64 = input.lines().map(snafu_to_decimal).sum();
    decimal_to_snafu(decimal_sum.try_into().unwrap())
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(25)?;
    dbg!(part1(input));

    Ok(())
}

#[test]
fn snafu_decimal() {
    assert_eq!(snafu_to_decimal("2=-1=0"), 4890);
}

#[test]
fn decimal_snafu() {
    assert_eq!(decimal_to_snafu(4890), "2=-1=0");
}

#[test]
fn example() {
    let input = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
    .trim();
    assert_eq!(part1(input), "2=-1=0");
}

#[test]
fn task() {
    let input = &read_input_to_string(25).unwrap();
    assert_eq!(part1(input), "20=2-02-0---02=22=21");
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(25).unwrap();
    b.iter(|| {
        part1(input);
    })
}

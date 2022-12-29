extern crate test;

use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

type MonkeyMap<'a> = HashMap<String, Operand<'a>>;

#[derive(Debug, Clone, Copy)]
enum Operand<'a> {
    Num(i64),
    Op(Operation<'a>),
}

impl<'a> Operand<'a> {
    fn value(&self, monkey_map: &MonkeyMap) -> i64 {
        match &self {
            Self::Num(value) => *value,
            Self::Op(operation) => operation.apply(monkey_map),
        }
    }
}

impl<'a> From<&str> for Operand<'a> {
    fn from(s: &str) -> Self {
        Operand::Num(s.parse().unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Div,
    Sub,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "-" => Operator::Sub,
            _ => panic!("Unknown operator {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Operation<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: Operator,
}

impl<'a> Operation<'a> {
    fn apply(&self, monkey_map: &MonkeyMap) -> i64 {
        let lhs = monkey_map[self.lhs].value(monkey_map);
        let rhs = monkey_map[self.rhs].value(monkey_map);
        match &self.op {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            Operator::Sub => lhs - rhs,
        }
    }
}

fn parse_input(input: &str, p2: bool) -> MonkeyMap {
    input
        .lines()
        .map(|line| {
            let (name, operand) = line.split_once(": ").unwrap();
            let operand = if let Some((lhs, op, rhs)) = operand.split_whitespace().collect_tuple() {
                if p2 && name == "root" {
                    Operand::Op(Operation {
                        lhs,
                        rhs,
                        op: Operator::Sub,
                    })
                } else {
                    Operand::Op(Operation {
                        lhs,
                        rhs,
                        op: op.into(),
                    })
                }
            } else {
                Operand::Num(operand.parse().unwrap())
            };
            (name.to_owned(), operand)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let map = parse_input(input, false);
    let root_op = &map["root"];
    root_op.value(&map) as usize
}

fn get_diff(insert_value: i64, map: &MonkeyMap) -> i64 {
    let mut map = map.to_owned();
    map.insert("humn".to_owned(), Operand::Num(insert_value));
    let root_op = &map["root"];
    root_op.value(&map)
}

fn signum(v: i64) -> i64 {
    match v.cmp(&0) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

fn next_try(lower_try: i64, upper_try: i64, map: &MonkeyMap) -> (i64, i64) {
    let lower_value = get_diff(lower_try, map);
    let upper_value = get_diff(upper_try, map);

    let lower_signum = signum(lower_value);
    let upper_signum = signum(upper_value);
    assert!(upper_signum != lower_signum);

    let prev_try_diff = upper_try - lower_try;
    let mid_try = lower_try + prev_try_diff / 2;
    let mid = get_diff(mid_try, map);
    let mid_signum = signum(mid);

    if mid_signum == lower_signum {
        (mid_try, upper_try)
    } else if mid_signum == upper_signum {
        (lower_try, mid_try)
    } else {
        (mid_try, mid_try)
    }
}

fn part2(input: &str, minus_one: bool) -> i64 {
    let map = parse_input(input, true);

    let mut lower_try = 0;
    let mut upper_try = 100;

    while signum(get_diff(lower_try, &map)) == signum(get_diff(upper_try, &map)) {
        lower_try = upper_try;
        upper_try *= 10;
    }

    while lower_try != upper_try {
        (lower_try, upper_try) = next_try(lower_try, upper_try, &map);
    }

    if minus_one {
        lower_try - 1
    } else {
        lower_try
    }
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(21)?;
    dbg!(part1(input));
    dbg!(part2(input, true));

    Ok(())
}

#[test]
fn example() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    assert_eq!(part1(input), 152);
    assert_eq!(part2(input, false), 301);
}

#[test]
fn task() {
    let input = &read_input_to_string(21).unwrap();
    assert_eq!(part1(input), 223971851179174);
    assert_eq!(part2(input, true), 3379022190351);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(21).unwrap();
        part1(input);
        part2(input, true);
    })
}

extern crate test;

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug)]
enum Operand {
    Num(i64),
    Old,
}

impl Operand {
    fn value(&self, old: i64) -> i64 {
        match &self {
            Self::Num(value) => *value,
            Self::Old => old,
        }
    }
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            digits => Operand::Num(digits.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Unknown operator {s}"),
        }
    }
}

#[derive(Debug)]
struct Operation {
    lhs: Operand,
    rhs: Operand,
    op: Operator,
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match &self.op {
            Operator::Add => self.lhs.value(old) + self.rhs.value(old),
            Operator::Mul => self.lhs.value(old) * self.rhs.value(old),
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        if let Some((lhs, op, rhs)) = s.split_whitespace().collect_tuple() {
            Operation {
                lhs: lhs.into(),
                rhs: rhs.into(),
                op: op.into(),
            }
        } else {
            panic!("Unknown Operation {s}")
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: i64,
    monkey_true: usize,
    monkey_false: usize,
}

impl Test {
    fn apply(&self, v: i64) -> usize {
        if v % self.divisor == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    n_inspections: usize,
}

impl Monkey {
    fn inspect(&self, big_divis: i64, p2: bool) -> Vec<(usize, i64)> {
        self.items
            .iter()
            .map(|v| {
                let v = if p2 {
                    self.operation.apply(*v) % big_divis
                } else {
                    self.operation.apply(*v) / 3
                };
                (self.test.apply(v), v)
            })
            .collect()
    }
}

fn parse_starting_items(line: &str) -> Vec<i64> {
    line.strip_prefix("Starting items: ")
        .unwrap_or_else(|| panic!("Unexpected starting items prefix for line {line}"))
        .split(", ")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    line.strip_prefix("Operation: new = ")
        .unwrap_or_else(|| panic!("Unexpected operation prefix for line {line}"))
        .into()
}

fn parse_test(lines: Vec<&str>) -> Test {
    Test {
        divisor: lines[0]
            .strip_prefix("Test: divisible by ")
            .unwrap_or_else(|| panic!("Unexpected test for line {}", lines[0]))
            .parse()
            .unwrap(),
        monkey_true: lines[1]
            .strip_prefix("If true: throw to monkey ")
            .unwrap_or_else(|| panic!("Unexpected true prefix for line {}", lines[1]))
            .parse()
            .unwrap(),
        monkey_false: lines[2]
            .strip_prefix("If false: throw to monkey ")
            .unwrap_or_else(|| panic!("Unexpected false prefix for line {}", lines[1]))
            .parse()
            .unwrap(),
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .map(|v| v.trim())
        .chunks(7)
        .into_iter()
        .map(|chunk| {
            let chunk: Vec<&str> = chunk.collect();
            Monkey {
                items: parse_starting_items(chunk[1]),
                operation: parse_operation(chunk[2]),
                test: parse_test(chunk[3..6].to_vec()),
                n_inspections: 0,
            }
        })
        .collect()
}

fn monkey_business(rounds: usize, mut monkeys: Vec<Monkey>, p2: bool) -> usize {
    let big_divis_number: i64 = monkeys.iter().map(|v| v.test.divisor).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let updates = monkeys[i].inspect(big_divis_number, p2);
            monkeys[i].items.clear();
            monkeys[i].n_inspections += updates.len();
            for (monkey_idx, v) in updates {
                monkeys[monkey_idx].items.push(v);
            }
        }
    }

    monkeys
        .iter()
        .map(|v| v.n_inspections)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .map(|(l, r)| l * r)
        .unwrap()
}

fn part1(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    monkey_business(20, monkeys, false)
}

fn part2(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    monkey_business(10_000, monkeys, true)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(11)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";
    assert_eq!(part1(input), 10605);
    assert_eq!(part2(input), 2713310158);
}

#[test]
fn task() {
    let input = &read_input_to_string(11).unwrap();
    assert_eq!(part1(input), 54253);
    assert_eq!(part2(input), 13119526120);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(11).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

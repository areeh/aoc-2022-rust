extern crate test;

use std::{cmp::Ordering, collections::VecDeque};

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    List(VecDeque<Node>),
    Leaf(u32),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let (mut left_q, mut right_q) = match (self.to_owned(), other.to_owned()) {
            (Node::List(left), Node::List(right)) => (left, right),
            _ => panic!("Expected to start with both nodes being lists"),
        };

        loop {
            let l = left_q.pop_front();
            let r = right_q.pop_front();

            let (l, r) = match (l, r) {
                (Some(l), Some(r)) => (l, r),
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                _ => return Ordering::Equal,
            };

            match (l.clone(), r.clone()) {
                (Node::Leaf(l), Node::Leaf(r)) => match l.cmp(&r) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                },
                (Node::Leaf(_), Node::List(_)) => {
                    left_q.push_front(Node::List(VecDeque::from(vec![l])));
                    right_q.push_front(r);
                }
                (Node::List(_), Node::Leaf(_)) => {
                    left_q.push_front(l);
                    right_q.push_front(Node::List(VecDeque::from(vec![r])));
                }
                (Node::List(_), Node::List(_)) => match l.cmp(&r) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                },
            }
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str) -> Node {
    let mut stack_me_some_brackets = VecDeque::new();
    let mut current_number = String::new();

    let mut node: Option<Node> = None;
    for c in line.chars() {
        if let Some(prev_node) = node {
            let current_node = stack_me_some_brackets.back_mut().unwrap();
            match current_node {
                Node::List(ref mut l) => l.push_back(prev_node),
                _ => panic!("Prev node with no current node"),
            };
        }
        node = match c {
            '[' => {
                stack_me_some_brackets.push_back(Node::List(VecDeque::new()));
                None
            }
            ']' => {
                let mut current_node = stack_me_some_brackets.pop_back();
                if !current_number.is_empty() {
                    match current_node {
                        Some(Node::List(ref mut l)) => {
                            l.push_back(Node::Leaf(current_number.parse().unwrap()))
                        }
                        _ => panic!("Number with no parent to put in"),
                    }
                    current_number = String::new();
                }
                current_node
            }
            ',' => {
                let current_node = stack_me_some_brackets.back_mut().unwrap();
                if !current_number.is_empty() {
                    match current_node {
                        Node::List(ref mut l) => {
                            l.push_back(Node::Leaf(current_number.parse().unwrap()))
                        }
                        _ => panic!("Number with no parent to put in"),
                    };
                }
                current_number = String::new();
                None
            }
            c @ '0'..='9' => {
                current_number.push(c);
                None
            }
            c => panic!("Expected bracket, comma, or number, got {c:?}"),
        };
    }
    node.unwrap()
}

fn part1(input: &str) -> usize {
    let mut right_order_idxs = Vec::new();
    input
        .lines()
        .chunks(3)
        .into_iter()
        .enumerate()
        .for_each(|(i, v)| {
            let v: Vec<&str> = v.collect();
            let left = parse_line(v[0]);
            let right = parse_line(v[1]);

            if left < right {
                right_order_idxs.push(i + 1);
            }
        });
    right_order_idxs.iter().sum()
}

fn part2(input: &str) -> usize {
    let divider1 = "[[2]]";
    let divider2 = "[[6]]";
    let mut nodes: Vec<Node> = input
        .lines()
        .chain(
            format!(
                "{divider1}
{divider2}"
            )
            .lines(),
        )
        .filter_map(|v| {
            if v.is_empty() {
                None
            } else {
                Some(parse_line(v))
            }
        })
        .collect();
    nodes.sort();

    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v == &parse_line(divider1) || v == &parse_line(divider2) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(13)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_parse1() {
    let line = "[[1,10,11],1,[1,10]]";
    assert_eq!(
        parse_line(line),
        Node::List(VecDeque::from(vec![
            Node::List(VecDeque::from(vec![
                Node::Leaf(1),
                Node::Leaf(10),
                Node::Leaf(11)
            ])),
            Node::Leaf(1),
            Node::List(VecDeque::from(vec![Node::Leaf(1), Node::Leaf(10)]))
        ]))
    )
}
#[test]
fn test_parse2() {
    let line = "[1,1,3,1,1]";
    assert_eq!(
        parse_line(line),
        Node::List(VecDeque::from(vec![
            Node::Leaf(1),
            Node::Leaf(1),
            Node::Leaf(3),
            Node::Leaf(1),
            Node::Leaf(1),
        ]))
    )
}

#[test]
fn example() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 140);
}

#[test]
fn task() {
    let input = &read_input_to_string(13).unwrap();
    assert_eq!(part1(input), 5882);
    assert_eq!(part2(input), 24948);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(13).unwrap();
        part1(input);
        part2(input);
    })
}

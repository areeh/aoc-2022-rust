use std::fs;

use bimap::BiMap;
use itertools::Itertools;

fn input1() -> std::io::Result<String> {
    fs::read_to_string("./src/day2/input.txt")
}

fn create_winning_matchup_map() -> BiMap<Move, Move> {
    let mut mp = BiMap::new();
    mp.insert(Move::Rock, Move::Scissors);
    mp.insert(Move::Paper, Move::Rock);
    mp.insert(Move::Scissors, Move::Paper);
    mp
}

fn get_loser(mv: &Move, win_map: &BiMap<Move, Move>) -> Move {
    win_map.get_by_left(mv).unwrap().clone()
}

fn get_winner(mv: &Move, win_map: &BiMap<Move, Move>) -> Move {
    win_map.get_by_right(mv).unwrap().clone()
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn outcome(&self, other: &Move, win_map: &BiMap<Move, Move>) -> Res {
        if self == other {
            Res::Draw
        } else if self.is_win(other, win_map) {
            Res::Win
        } else {
            Res::Lose
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn is_win(&self, rhs_mv: &Move, win_map: &BiMap<Move, Move>) -> bool {
        self == &get_winner(rhs_mv, win_map)
    }

    fn pick_move(&self, required_outcome: &Res, win_map: &BiMap<Move, Move>) -> Move {
        match required_outcome {
            Res::Draw => self.clone(),
            Res::Lose => get_loser(self, win_map),
            Res::Win => get_winner(self, win_map),
        }
    }
}

enum Res {
    Win,
    Draw,
    Lose,
}

impl Res {
    fn score(&self) -> u32 {
        match self {
            Res::Win => 6,
            Res::Draw => 3,
            Res::Lose => 0,
        }
    }
}

fn parse_required_outcome(s: &str) -> Res {
    match s {
        "X" => Res::Lose,
        "Y" => Res::Draw,
        "Z" => Res::Win,
        _ => panic!("Unexpected should res {s}"),
    }
}

fn parse_lhs_move(s: &str) -> Move {
    match s {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Unexpected lhs move {s}"),
    }
}

fn parse_rhs_move(s: &str) -> Move {
    match s {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Unexpected rhs move {s}"),
    }
}

fn parse_line_p1(line: &str) -> (Move, Move) {
    if let Some((lhs, rhs)) = line.split_whitespace().collect_tuple() {
        (parse_lhs_move(lhs), parse_rhs_move(rhs))
    } else {
        panic!("Expected exactly 2 moves, got {}", line)
    }
}

fn score_match_p1((lhs, rhs): (Move, Move), win_map: &BiMap<Move, Move>) -> u32 {
    rhs.score() + rhs.outcome(&lhs, win_map).score()
}

fn part1(moves: String) -> u32 {
    let win_map = create_winning_matchup_map();
    moves
        .lines()
        .map(|line| score_match_p1(parse_line_p1(line), &win_map))
        .sum()
}

fn parse_line_p2(line: &str) -> (Move, Res) {
    if let Some((lhs, rhs)) = line.split_whitespace().collect_tuple() {
        (parse_lhs_move(lhs), parse_required_outcome(rhs))
    } else {
        panic!("Expected exactly 2 strings, got {}", line)
    }
}

fn score_match_p2((lhs, rhs): (Move, Res), win_map: &BiMap<Move, Move>) -> u32 {
    rhs.score() + lhs.pick_move(&rhs, win_map).score()
}

fn part2(moves: String) -> u32 {
    let win_map = create_winning_matchup_map();
    moves
        .lines()
        .map(|line| score_match_p2(parse_line_p2(line), &win_map))
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
    let input = "A Y
B X
C Z"
    .to_owned();
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input), 12);
}

#[test]
fn task() {
    let input = input1().unwrap();
    assert_eq!(part1(input.clone()), 14163);
    assert_eq!(part2(input), 12091);
}

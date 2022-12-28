extern crate test;

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_line(line: &str) -> Blueprint {
    let (id, recipe) = line.split_once(": ").unwrap();
    let recipe = recipe.split(". ").collect_vec();

    let obsidian_recipe = recipe[2]
        .strip_prefix("Each obsidian robot costs ")
        .unwrap()
        .split(" and ")
        .collect_vec();

    let geode_recipe = recipe[3]
        .strip_prefix("Each geode robot costs ")
        .unwrap()
        .split(" and ")
        .collect_vec();

    // dbg!(&geode_recipe);

    Blueprint {
        id: id.strip_prefix("Blueprint ").unwrap().parse().unwrap(),
        ore_robot: Resources {
            ore: recipe[0]
                .strip_prefix("Each ore robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        clay_robot: Resources {
            ore: recipe[1]
                .strip_prefix("Each clay robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        obsidian_robot: Resources {
            ore: obsidian_recipe[0]
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap(),
            clay: obsidian_recipe[1]
                .strip_suffix(" clay")
                .unwrap()
                .parse()
                .unwrap(),
            obsidian: 0,
            geode: 0,
        },
        geode_robot: Resources {
            ore: geode_recipe[0]
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap(),
            clay: 0,
            obsidian: geode_recipe[1]
                .strip_suffix(" obsidian.")
                .unwrap()
                .parse()
                .unwrap(),
            geode: 0,
        },
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn cost(&self, robot: &Robot) -> Resources {
        match robot {
            Robot::Ore => self.ore_robot,
            Robot::Clay => self.clay_robot,
            Robot::Obsidian => self.obsidian_robot,
            Robot::Geode => self.geode_robot,
            Robot::Nothing => Resources::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Resources {
    geode: isize,
    obsidian: isize,
    clay: isize,
    ore: isize,
}

impl Resources {
    fn new() -> Self {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn all_greater_equal(&self, other: Resources) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<isize> for Resources {
    type Output = Self;

    fn mul(self, other: isize) -> Self::Output {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    stock: Resources,
    income: Resources,
    remaining: usize,
    prev_robot: Robot,
    prev_stock: Resources,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Assuming Resources ordering is geode > obsidian > clay > ore
        // this will be correct for the last step and useful for the rest
        cumulative_resources(&self.stock, &self.income, 1).cmp(&cumulative_resources(
            &other.stock,
            &other.income,
            1,
        ))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(remaining: usize) -> Self {
        Self {
            stock: Resources::new(),
            income: Resources {
                geode: 0,
                obsidian: 0,
                clay: 0,
                ore: 1,
            },
            remaining,
            prev_robot: Robot::Nothing,
            prev_stock: Resources::new(),
        }
    }

    fn update_state(&self, blueprint: &Blueprint) -> Vec<Self> {
        // A robot created at the last timestep will not produce anything
        if self.remaining == 1 {
            return vec![Self {
                income: self.income,
                stock: cumulative_resources(&self.stock, &self.income, 1),
                remaining: 0,
                prev_robot: Robot::Nothing,
                prev_stock: self.stock,
            }];
        }

        // Always optimal to make a geode robot if we can
        if self.stock.all_greater_equal(blueprint.cost(&Robot::Geode)) {
            let stock = self.stock + self.income;
            let remaining = self.remaining - 1;
            let (stock, income) = make_robot(&Robot::Geode, blueprint, &stock, &self.income);
            return vec![Self {
                stock,
                income,
                remaining,
                prev_stock: self.stock,
                prev_robot: Robot::Geode,
            }];
        }

        let mut new_states = Vec::new();
        for robot in ROBOTS {
            // Never optimal to delay a robot we could have made last turn
            if self.prev_robot == Robot::Nothing
                && robot != Robot::Nothing
                && self.prev_stock.all_greater_equal(blueprint.cost(&robot))
            {
                continue;
            }

            if self.stock.all_greater_equal(blueprint.cost(&robot)) {
                let stock = self.stock + self.income;
                let remaining = self.remaining - 1;
                let (stock, income) = make_robot(&robot, blueprint, &stock, &self.income);
                new_states.push(Self {
                    stock,
                    income,
                    remaining,
                    prev_stock: self.stock,
                    prev_robot: robot,
                })
            }
        }
        new_states
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input.lines().map(parse_line).collect()
}

fn cumulative_resources(stock: &Resources, income: &Resources, remaining: usize) -> Resources {
    *stock + (*income * remaining as isize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
    Nothing,
}

impl Robot {
    fn income(&self) -> Resources {
        match self {
            Robot::Nothing => Resources {
                geode: 0,
                obsidian: 0,
                clay: 0,
                ore: 0,
            },
            Robot::Ore => Resources {
                geode: 0,
                obsidian: 0,
                clay: 0,
                ore: 1,
            },
            Robot::Clay => Resources {
                geode: 0,
                obsidian: 0,
                clay: 1,
                ore: 0,
            },
            Robot::Obsidian => Resources {
                geode: 0,
                obsidian: 1,
                clay: 0,
                ore: 0,
            },
            Robot::Geode => Resources {
                geode: 1,
                obsidian: 0,
                clay: 0,
                ore: 0,
            },
        }
    }
}

fn make_robot(
    kind: &Robot,
    blueprint: &Blueprint,
    stock: &Resources,
    income: &Resources,
) -> (Resources, Resources) {
    (*stock - blueprint.cost(kind), *income + kind.income())
}

const ROBOTS: [Robot; 5] = [
    Robot::Geode,
    Robot::Obsidian,
    Robot::Clay,
    Robot::Ore,
    Robot::Nothing,
];

fn parts(input: &str, remaining: usize, p2: bool) -> usize {
    let mut blueprints = parse_blueprints(input);
    if p2 {
        blueprints = blueprints.into_iter().take(3).collect_vec();
    }
    let best_score: Vec<_> = blueprints
        .iter()
        .map(|blueprint| {
            let mut queue = vec![State::new(remaining)];

            for _ in 0..remaining {
                let mut new_queue = Vec::new();
                while let Some(state) = queue.pop() {
                    new_queue.extend(state.update_state(blueprint));
                }

                new_queue.sort_by(|a, b| b.cmp(a));
                new_queue = new_queue.into_iter().take(500).collect_vec();

                queue = new_queue;
            }

            queue.sort_by(|a, b| b.stock.geode.cmp(&a.stock.geode));
            let value = if let Some(state) = queue.get(0) {
                assert_eq!(state.remaining, 0);
                state.stock.geode
            } else {
                0
            };

            if p2 {
                value as usize
            } else {
                blueprint.id * value as usize
            }
        })
        .collect();
    if p2 {
        best_score.iter().product()
    } else {
        best_score.iter().sum()
    }
}

fn part1(input: &str) -> usize {
    parts(input, 24, false)
}

fn part2(input: &str) -> usize {
    parts(input, 32, true)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(19)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_eq!(part1(input), 33);
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    assert_eq!(part2(input), 56);
    let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_eq!(part2(input), 62);
}

#[test]
fn task() {
    let input = &read_input_to_string(19).unwrap();
    assert_eq!(part1(input), 1395);
    assert_eq!(part2(input), 2700);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(19).unwrap();
        part1(input);
        part2(input);
    })
}

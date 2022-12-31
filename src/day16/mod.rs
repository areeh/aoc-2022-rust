extern crate test;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::Sub,
};

use itertools::Itertools;
use petgraph::{algo::floyd_warshall, prelude::NodeIndex, prelude::UnGraph, visit::Bfs};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_node(line: &str) -> (&str, usize, Vec<&str>) {
    let (room, connections) = line
        .split_once("; tunnels lead to valves ")
        .unwrap_or_else(|| {
            line.split_once("; tunnel leads to valve ")
                .unwrap_or_else(|| panic!("Could not split line {line}"))
        });
    let (name, value) = room.split_once(" has flow rate=").unwrap();
    (
        name.strip_prefix("Valve ").unwrap(),
        value.parse().unwrap(),
        connections.split(", ").collect(),
    )
}

type Cave = UnGraph<(usize, String), usize>;

fn parse_nodes(input: &str) -> Cave {
    let mut name_map = HashMap::new();
    let mut cave = UnGraph::<(usize, String), usize>::default();
    for (name, flow, _) in input.lines().map(parse_node) {
        name_map.insert(name, cave.add_node((flow, name.into())));
    }
    for (name, _, neighbors) in input.lines().map(parse_node) {
        for other in neighbors {
            cave.update_edge(name_map[name], name_map[other], 1usize);
        }
    }
    // println!("{:?}", Dot::new(&cave));

    cave
}

fn prune_nodes(start: NodeIndex, cave: &Cave) -> Cave {
    // Could've just done this in the shortest paths step
    let mut cave = cave.clone();
    let mut bfs = Bfs::new(&cave, start);
    while let Some(nx) = bfs.next(&cave) {
        if cave[nx].0 == 0 && cave[nx].1 != "AA" {
            for pair in cave.clone().neighbors(nx).permutations(2) {
                let dist = cave[cave.find_edge(pair[0], nx).unwrap()]
                    + cave[cave.find_edge(pair[1], nx).unwrap()];
                if let Some(edge) = cave.find_edge(pair[0], pair[1]) {
                    if dist < cave[edge] {
                        cave[edge] = dist;
                    }
                } else {
                    cave.add_edge(pair[0], pair[1], dist);
                }
            }
        }
    }

    cave.retain_nodes(|graph, nx| graph[nx].0 != 0 || graph[nx].1 == "AA");
    cave
}

fn visit(
    position: &NodeIndex,
    remaining: usize,
    released: usize,
    closed_valves: &BTreeSet<NodeIndex>,
    flow: usize,
    cave: &Cave,
    distance_map: &HashMap<(NodeIndex, NodeIndex), usize>,
) -> usize {
    if closed_valves.is_empty() || remaining == 0 {
        return released + (flow * remaining);
    }

    closed_valves
        .clone()
        .iter()
        .map(|candidate| {
            let cost = distance_map[&(*position, *candidate)] + 1;
            if remaining < cost {
                released + (flow * remaining)
            } else {
                let mut closed_valves = closed_valves.clone();
                closed_valves.remove(candidate);
                visit(
                    candidate,
                    remaining - cost,
                    released + (cost * flow),
                    &closed_valves,
                    flow + cave[*candidate].0,
                    cave,
                    distance_map,
                )
            }
        })
        .max()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let cave = parse_nodes(input);
    let start = cave.node_indices().find(|nx| cave[*nx].1 == *"AA").unwrap();
    let cave = prune_nodes(start, &cave);
    let start = cave.node_indices().find(|nx| cave[*nx].1 == *"AA").unwrap();
    // println!("{:?}", Dot::new(&cave));

    let distance_map = floyd_warshall(&cave, |edge| *edge.weight()).unwrap();
    let closed_valves = cave.node_indices().unique().collect();
    visit(&start, 30, 0, &closed_valves, 0, &cave, &distance_map)
}

fn part2(input: &str) -> usize {
    let cave = parse_nodes(input);
    let start = cave.node_indices().find(|nx| cave[*nx].1 == *"AA").unwrap();
    let cave = prune_nodes(start, &cave);
    let start = cave.node_indices().find(|nx| cave[*nx].1 == *"AA").unwrap();
    // println!("{:?}", Dot::new(&cave));

    let distance_map = floyd_warshall(&cave, |edge| *edge.weight()).unwrap();
    let closed_valves = cave.node_indices().unique().collect_vec();

    let mut solutions: HashMap<BTreeSet<NodeIndex>, usize> = HashMap::new();
    let subsets = closed_valves.clone().into_iter().powerset().collect_vec();
    let subsets: Vec<BTreeSet<NodeIndex>> = subsets
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect();

    for possible in subsets {
        solutions.insert(
            possible.clone(),
            visit(&start, 26, 0, &possible, 0, &cave, &distance_map),
        );
    }

    let all_valves_set: HashSet<&NodeIndex> = closed_valves.iter().collect();
    solutions
        .keys()
        .map(|solution| {
            let solution_set: HashSet<&NodeIndex> = solution.iter().collect();
            let other = all_valves_set.sub(&solution_set);

            let lhs: BTreeSet<NodeIndex> = solution_set.iter().map(|v| **v).collect();
            let rhs: BTreeSet<NodeIndex> = other.iter().map(|v| **v).collect();
            solutions[&lhs] + solutions[&rhs]
        })
        .max()
        .unwrap()
}

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(16)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_prune() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    let cave = parse_nodes(input);
    let start = cave.node_indices().find(|nx| cave[*nx].1 == *"AA").unwrap();
    // println!("{:?}", Dot::new(&cave));
    let _cave = prune_nodes(start, &cave);
    // println!("{:?}", Dot::new(&cave));
}

#[test]
fn example() {
    let input = "Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    assert_eq!(part1(input), 1651);
    assert_eq!(part2(input), 1707);
}

#[test]
fn task() {
    let input = &read_input_to_string(16).unwrap();
    assert_eq!(part1(input), 1376);
    // assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    let input = &read_input_to_string(16).unwrap();
    b.iter(|| {
        part1(input);
        part2(input);
    })
}

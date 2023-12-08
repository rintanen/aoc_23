use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    left: String,
    right: String,
}

fn parse_node(line: &str) -> (String, Node) {
    let mut iter = line.split(" = ");
    let name = iter.next().unwrap();

    let mut iter = iter.next().unwrap()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(", ");
    let left = iter.next().unwrap();
    let right = iter.next().unwrap();
    let node = Node {
        left: left.to_string(),
        right: right.to_string(),
    };
    (name.to_string(), node)
}


fn pt1_walk_tree_from_to(nodes: &HashMap<String, Node>, from: &Node, to: &Node, directions: &Vec<char>) -> u32 {
    let mut steps = 0;
    let mut node = from;
    for direction in directions.iter().cycle() {
        node = match direction {
            'L' => nodes.get(&node.left).unwrap(),
            'R' => nodes.get(&node.right).unwrap(),
            _ => panic!("Invalid direction: {}", direction),
        };
        steps += 1;
        if node == to {
            break;
        }
    }
    steps
}

fn pt2_walk_tree_from_to(nodes: &HashMap<String, Node>, from: &Vec<&Node>, to: &HashSet<&Node>, directions: &Vec<char>) -> u32 {
    let mut steps = 0;
    let mut current_nodes = from.clone();
    for direction in directions.iter().cycle() {
        current_nodes = match direction {
            'L' => current_nodes.iter().map(|node| nodes.get(&node.left).unwrap()).collect(),
            'R' => current_nodes.iter().map(|node| nodes.get(&node.right).unwrap()).collect(),
            _ => panic!("Invalid direction: {}", direction),
        };
        steps += 1;
        if current_nodes.iter().all(|node| to.contains(*node)) {
            break;
        }
    }
    steps
}

fn main() {
    let input = include_str!("../../inputs/day08.in");
    let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let nodes = input
        .lines()
        .skip(2)
        .map(|line| parse_node(line))
        .collect::<HashMap<String, Node>>();
    // let steps = pt1_walk_tree_from_to(&nodes, &nodes["AAA"], &nodes["ZZZ"], &instructions);
    // println!("pt1: {}", steps);

    let starts = nodes.iter().filter(|(name, node)| name.ends_with('A')).map(|(_, node)| node).collect::<Vec<_>>();
    let ends = nodes.iter().filter(|(name, node)| name.ends_with('Z')).map(|(_, node)| node).collect::<HashSet<_>>();
    // dbg!(starts);
    // dbg!(ends);
    let steps = pt2_walk_tree_from_to(&nodes, &starts, &ends, &instructions);
    println!("pt2: {}", steps);
}
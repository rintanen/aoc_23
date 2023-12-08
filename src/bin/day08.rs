use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn parse_node(line: &str) -> (String, Node) {
    let mut iter = line.split(" = ");
    let name = iter.next().unwrap();
    let mut it = iter.next().unwrap()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(", ");
    let left = it.next().unwrap();
    let right = it.next().unwrap();
    let node = Node {
        left: left.to_string(),
        right: right.to_string(),
    };
    (name.to_string(), node)
}

fn main() {
    let input = include_str!("../../inputs/day08.in");
    let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    println!("pt1: {:?}", instructions);
    let nodes = input
        .lines()
        .skip(2)
        .map(|line| parse_node(line))
        .collect::<HashMap<String, Node>>();
    dbg!(nodes);
}
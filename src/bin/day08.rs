use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    name: String,
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
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    };
    (name.to_string(), node)
}


fn walk_tree_from_to(nodes: &HashMap<String, Node>, from: &Node, to: &str, directions: &Vec<char>) -> usize {
    let mut node = from;
    let steps = directions.iter().cycle().position(|direction| {
        node = match direction {
            'L' => nodes.get(&node.left).unwrap(),
            'R' => nodes.get(&node.right).unwrap(),
            _ => panic!("Invalid direction: {}", direction),
        };
        node.name.ends_with(to)
    });
    steps.unwrap() + 1
}


fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}


fn main() {
    let input = include_str!("../../inputs/day08.in");
    let instructions = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let nodes = input
        .lines()
        .skip(2)
        .map(|line| parse_node(line))
        .collect::<HashMap<String, Node>>();
    let steps = walk_tree_from_to(&nodes, &nodes["AAA"], "ZZZ", &instructions);
    println!("pt1: {}", steps);

    let starts = nodes.iter().filter(|(name, _node)| name.ends_with('A')).map(|(_, node)| node).collect::<Vec<_>>();

    let steps = starts.iter()
        .map(|node| walk_tree_from_to(&nodes, &node, "Z", &instructions))
        .fold(1, |ans, x| (x*ans) / gcd(x,ans));
    println!("pt2: {}", steps);
}

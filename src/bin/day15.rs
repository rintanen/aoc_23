use std::collections::{BTreeMap, HashMap};

struct Box {

}

fn hash(word: &[u8]) -> u64 {
    word.iter().fold(0, |hash, &w| (hash + w as u64) * 17 % 256)
}


fn main() {
    let input = include_str!("../../inputs/day15.in");
    let sum = input.split(',').map(|word| hash(word.as_bytes())).sum::<u64>();
    dbg!(sum);
    HashMap::with_capacity(256);
    BTreeMap::new();

}
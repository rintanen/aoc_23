use std::collections::HashSet;

use itertools::iproduct;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    character: char,
    location: (usize, usize)
}

impl Symbol {
    fn new(character: char, location: (usize, usize)) -> Self {
        Self {character, location}
    }
}

#[derive(Debug)]
struct EnginePart {
    num: u32,
    digits: usize,
    location: (usize, usize),
}

impl EnginePart {
    fn new(num: u32, digits: usize, location: (usize, usize)) -> Self {
        Self {num, digits, location}
    }

    fn adjacent(&self) -> Vec<(usize, usize)> {
        let rectangle = iproduct!(
            self.location.0.checked_sub(1).unwrap_or(self.location.0)..=self.location.0 + self.digits,
            self.location.1.checked_sub(1).unwrap_or(self.location.1)..=self.location.1 + 1
        ).collect::<Vec<_>>();
        rectangle.iter().cloned()
            .filter(|(x, y)| {!(
                *x >= self.location.0 &&
                *x <= self.location.0 + self.digits - 1 &&
                *y == self.location.1
            )
            })
            .collect::<Vec<_>>()
    }
}

fn split_for_symbols(c: char) -> bool {
    c == '.' || c.is_numeric()
}

fn get_engine_parts_from_line(y: usize, engine_parts: Vec<(usize, &str)>) -> Vec<EnginePart> {
    engine_parts.iter()
        .map(|(idx, part_value)| EnginePart::new(part_value.parse::<u32>().unwrap(), part_value.len(), (*idx, y)))
        .collect::<Vec<_>>()
}

fn get_symbols_from_line(line: &str, y: usize, symbols_from_line: HashSet<&str>) -> HashSet<Symbol> {
    let mut symbols: HashSet<Symbol> = HashSet::new();
    for symbol in symbols_from_line.iter() {
        symbols.extend(
            line
                .match_indices(symbol)
                .map(|(idx, character)| {
                    let symbol_char = character.chars().next().unwrap();
                    Symbol::new(symbol_char, (idx, y))
                })
        )
    }
    symbols
}

fn find_numbers(input: &str) -> Vec<(usize, &str)> {
    let mut result = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for mat in re.find_iter(input) {
        let start_idx = mat.start();
        let number = mat.as_str();
        result.push((start_idx, number));
    }
    result
}

fn find_adjacent_engine_parts<'a>(symbol: &Symbol, engine_parts: &'a Vec<EnginePart>) -> Option<Vec<&'a EnginePart>>{
    let found_neighbours = engine_parts.iter()
        .filter(|engine_part| engine_part.location.1.saturating_sub(symbol.location.1) <= 1)
        .filter(|engine_part| engine_part.adjacent().iter().any(|location| symbol.location == *location))
        .collect::<Vec<_>>();
    if found_neighbours.len() == 2 {
        return Some(found_neighbours);
    }
    Noneadd
}


fn main() {
    let input = include_str!("../../inputs/day03.in");

    let mut engine_parts: Vec<EnginePart> = Vec::with_capacity(1000);
    let mut symbols: Vec<Symbol> = Vec::with_capacity(1000);
    for (y, line) in input.lines().enumerate() {
        let engine_part_items_items = find_numbers(line);
        let symbol_items = line.split(|c: char| split_for_symbols(c)).filter(|w| !w.is_empty()).collect::<HashSet<&str>>();

        engine_parts.extend(get_engine_parts_from_line(y, engine_part_items_items));
        symbols.extend(get_symbols_from_line(line, y, symbol_items));
    }
    let symbol_locations: HashSet<(usize, usize)> = symbols.iter().map(|s| s.location).collect();

    // pt1
    let sum = engine_parts.iter()
        .filter(|engine_part| engine_part.adjacent().iter().any(|location| symbol_locations.contains(location)))
        .map(|engine_part| engine_part.num)
        .sum::<u32>();
    println!("Sum of engine parts adjacent to symbols: {}", sum);

    // pt2
    let pt2 = symbols.iter()
        .filter(|symbol| symbol.character == '*')
        .filter_map(|symbol| find_adjacent_engine_parts(symbol, &engine_parts))
        .map(|engine_parts| engine_parts.iter().map(|engine_part| engine_part.num).product::<u32>())
        .sum::<u32>();
    println!("sum of gear ratios: {}", pt2);
}


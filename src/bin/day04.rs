use std::collections::HashSet;

struct Card {
    correct_numbers: HashSet<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(card_string: &str) -> Self {
        let cutoff = card_string.find(':').map(|i| i + 1).unwrap_or(0);
        let card_string = &card_string[cutoff..].trim();

        let mut card_iter = card_string.split(" | ");

        let correct_numbers = card_iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let my_numbers = card_iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Self {
            correct_numbers,
            my_numbers,
        }
    }

    fn number_of_matches(&self) -> u32 {
        self.my_numbers.iter()
            .filter(|num| self.correct_numbers.contains(num))
            .count()
            .try_into()
            .unwrap()
    }
}

fn pt2_calculate_number_of_cards(matches: &u32, all_matches: &[u32], card_number: usize) -> u32 {
    let mut matches = *matches;
    for n in card_number+1..card_number+1+matches as usize {
        matches += pt2_calculate_number_of_cards(&all_matches[n], all_matches, n);
    }
    matches
}

fn worth(matches: u32) -> u32 {
    if matches == 0 {
        return 0;
    }
    2u32.pow(matches - 1)
}


fn main() {
    let input = include_str!("../../inputs/day04.in");
    let all_matches = input.lines()
        .map(|line| Card::new(line).number_of_matches())
        .collect::<Vec<_>>();
    let pt1 = all_matches.iter().fold(0, |acc, matches| acc + worth(*matches));
    println!("pt1: {}", pt1);
    let pt2 = all_matches.iter().enumerate()
        .map(|(i, matches)| 1 + pt2_calculate_number_of_cards(matches, &all_matches, i))
        .sum::<u32>();
    println!("pt2: {}", pt2);
}

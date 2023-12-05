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

    fn worth(&self) -> u32 {
        if self.number_of_matches() == 0 {
            return 0;
        }
        2u32.pow(self.number_of_matches() - 1)
    }
}


fn pt2_calculate_number_of_cards(card: &Card, all_cards: &[Card], card_number: usize) -> u32 {
    let mut n_matches = card.number_of_matches();
    if n_matches == 0 {
        return n_matches;
    }
    for n in card_number..card_number + n_matches as usize {
        n_matches += pt2_calculate_number_of_cards(&all_cards[n], all_cards, n);
    }
    n_matches
}



fn main() {
    let input = include_str!("../../inputs/day04.in");

    let pt1 = input.lines()
        .map(|line| Card::new(line))
        .fold(0, |acc, card| acc + card.worth());
    println!("pt1: {}", pt1);


    let all_cards = input.lines()
        .map(|line| Card::new(line))
        .collect::<Vec<_>>();
}
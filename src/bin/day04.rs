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
        let matches = self.number_of_matches();
        if matches == 0 {
            return 0;
        }
        2u32.pow(matches - 1)
    }
}


fn pt2_calculate_number_of_cards(card: &Card, all_cards: &[Card], card_number: usize) -> u32 {
    let mut matches = card.number_of_matches();
    for n in card_number+1..card_number+1+matches as usize {
        matches += pt2_calculate_number_of_cards(&all_cards[n], all_cards, n);
    }
    matches
}


fn main() {
    /*
    refaktrointi
    muuta rakenne niin että lasketaan vaan suoraan montako matchia niin ei tarvi uudestaan ja uudestaan
    */
    let input = include_str!("../../inputs/day04.in");
    let cards = input.lines().map(|line| Card::new(line)).collect::<Vec<_>>();

    let pt1 = cards.iter().fold(0, |acc, card| acc + card.worth());
    println!("pt1: {}", pt1);

    let pt2 = cards.iter().enumerate()
        .map(|(i, card)| 1 + pt2_calculate_number_of_cards(card, &cards, i))
        .sum::<u32>();
    println!("pt2: {}", pt2);
}
use std::collections::HashSet;
use itertools::all;

fn split_numbers(card_string: &str) -> (HashSet<u32>, Vec<u32>) {
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

    (correct_numbers, my_numbers)

}

fn number_of_matches(correct_numbers: &HashSet<u32>, my_numbers: &Vec<u32>) -> u32 {
    my_numbers.iter()
        .filter(|num| correct_numbers.contains(num))
        .count()
        .try_into()
        .unwrap()
}

fn calculate_worth(matches: u32) -> u32 {
    if matches == 0 {
        return 0;
    }
    2u32.pow(matches - 1)
}

fn calculate_worth_pt2(
    correct_numbers: &HashSet<u32>, my_numbers: &Vec<u32>, all_cards: &Vec<(HashSet<u32>, Vec<u32>)>, card_number: usize
) -> u32 {
    let num_matches = number_of_matches(correct_numbers, my_numbers);
    let mut worth = calculate_worth(num_matches);
    for _ in card_number..=card_number+num_matches as usize {
        let (correct_numbers, my_numbers) = all_cards[card_number];
        let num_matches = number_of_matches(&correct_numbers, &my_numbers);
        worth += calculate_worth(num_matches);
    }
    worth
}

fn main() {
    let input = include_str!("../../inputs/day04.in");

    let pt1 = input.lines()
        .map(|line| split_numbers(line))
        .map(|(correct_numbers, my_numbers)| number_of_matches(&correct_numbers, &my_numbers))
        .fold(0, |acc, matches| acc + calculate_worth(matches));
    println!("pt1: {}", pt1);


    let all_cards = input.lines()
        .map(|line| split_numbers(line))
        .collect::<Vec<_>>();

    let pt2 = all_cards.iter().enumerate()
        .map(|(idx, (correct_numbers, my_numbers))|
            calculate_worth_pt2(correct_numbers, my_numbers, &all_cards, idx)
        )
        .sum::<u32>();
    println!("pt2: {}", pt2);

}
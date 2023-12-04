use std::collections::HashSet;

fn number_of_matches(correct_numbers: HashSet<u32>, my_numbers: Vec<u32>) -> u32 {
    my_numbers.iter()
        .filter(|num| correct_numbers.contains(num))
        .count()
        .try_into()
        .unwrap()
}


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

fn calculate_worth(matches: u32) -> u32 {
    if matches == 0 {
        return 0;
    }
    2u32.pow(matches - 1)
}

fn main() {
    let input = include_str!("../../inputs/day04.in");

    let pt1 = input.lines()
        .map(|line| split_numbers(line))
        .map(|(correct_numbers, my_numbers)| number_of_matches(correct_numbers, my_numbers))
        .fold(0, |acc, matches| acc + calculate_worth(matches));
    println!("pt1: {}", pt1);


}
use itertools::Itertools;

#[derive(Debug)]
enum Rules {
    NoJokers,
    Jokers,
}

#[derive(Debug)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    rules: Rules,
}

impl Hand {
    fn value(&self) -> [u32; 5] {
        let rank = match self.rules {
            Rules::NoJokers => rank_without_joker_rule(self.cards.as_ref()),
            Rules::Jokers => rank_with_joker_rule(self.cards.as_ref()),
        };

        let score = {
            match rank {
                Rank::FiveOfAKind => 100_000_000,
                Rank::FourOfAKind => 10_000_000,
                Rank::FullHouse => 1_000_000,
                Rank::ThreeOfAKind => 100_000,
                Rank::TwoPairs => 10_000,
                Rank::OnePair => 1000,
                Rank::HighCard => 0,
            }
        };
        [
            score + self.cards[0],
            score + self.cards[1],
            score + self.cards[2],
            score + self.cards[3],
            score + self.cards[4]
        ]
    }
}

fn rank_without_joker_rule(cards: &[u32]) -> Rank {
    if is_five_of_a_kind(cards) {
        Rank::FiveOfAKind
    } else if is_four_of_a_kind(cards) {
        Rank::FourOfAKind
    } else if is_full_house(cards) {
        Rank::FullHouse
    } else if is_three_of_a_kind(cards) {
        Rank::ThreeOfAKind
    } else if is_two_pairs(cards) {
        Rank::TwoPairs
    } else if is_one_pair(cards) {
        Rank::OnePair
    } else {
        Rank::HighCard
    }
}


fn rank_with_joker_rule(cards: &[u32]) -> Rank {
    let num_jokers = cards.iter().filter(|card| **card == 1).count();
    if num_jokers == 0 {
        return rank_without_joker_rule(cards.as_ref());
    }
    let cards_without_jokers = cards.iter().cloned().filter(|card| *card != 1).collect::<Vec<u32>>();
    let rank_without_jokers = rank_without_joker_rule(cards_without_jokers.as_ref());
    let rank_with_jokers = match rank_without_jokers {
        Rank::FiveOfAKind => Rank::FiveOfAKind,
        Rank::FourOfAKind => Rank::FiveOfAKind,
        Rank::ThreeOfAKind =>
            match num_jokers {
                1 => Rank::FourOfAKind,
                2 => Rank::FiveOfAKind,
                _ => panic!("Invalid number of jokers (has to be 1 or 2): {}", num_jokers),
            },
        Rank::TwoPairs => Rank::FullHouse,
        Rank::OnePair =>
            match num_jokers {
                1 => Rank::ThreeOfAKind,
                2 => Rank::FourOfAKind,
                3 => Rank::FiveOfAKind,
                _ => panic!("Invalid number of jokers (has to be between 1-3): {}", num_jokers),
            },
        Rank::HighCard =>
            match num_jokers {
                1 => Rank::OnePair,
                2 => Rank::ThreeOfAKind,
                3 => Rank::FourOfAKind,
                4 => Rank::FiveOfAKind,
                _ => panic!("Invalid number of jokers (has to be between 1-4): {}", num_jokers),
            },
        _ => panic!("Invalid rank (fullhouse not possible): {:?}", rank_without_jokers),
    };
    rank_with_jokers
}

fn is_five_of_a_kind(cards: &[u32]) -> bool {
    cards.iter().all(|card| *card == cards[0])
}

fn is_four_of_a_kind(cards: &[u32]) -> bool {
    cards.iter().any(|card| cards.iter().filter(|c| **c == *card).count() == 4)
}

fn is_full_house(cards: &[u32]) -> bool {
    is_three_of_a_kind(cards) && is_one_pair(cards)
}

fn is_three_of_a_kind(cards: &[u32]) -> bool {
    cards.iter().any(|card| cards.iter().filter(|c| **c == *card).count() == 3)
}

fn is_two_pairs(cards: &[u32]) -> bool {
    let mut pairs = vec![];
    for card in cards {
        if pairs.contains(card) {
            continue;
        }
        if cards.iter().filter(|c| **c == *card).count() == 2 {
            pairs.push(*card);
        }
    }
    pairs.len() == 2
}

fn is_one_pair(cards: &[u32]) -> bool {
    cards.iter().any(|card| cards.iter().filter(|c| **c == *card).count() == 2)
}


fn map_card_to_value(card: char, rules: &Rules) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => match rules {Rules::NoJokers => 11, Rules::Jokers => 1},
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Invalid card: {}", card),
    }
}

fn parse_hand(line: &str, rules: Rules) -> Hand {
    let mut hand_iter = line.split_whitespace();
    Hand {
        cards: hand_iter.next().unwrap().chars().map(|c| map_card_to_value(c, &rules)).collect(),
        bid: hand_iter.next().unwrap().parse::<u32>().unwrap(),
        rules,
    }
}

fn calculate_winnings(hands: &[Hand]) -> u32 {
    let sorted_hands = hands.iter()
        .map(|hand| (hand.value(), hand.bid))
        .sorted_by_key(|(value, _)| *value)
        .collect::<Vec<_>>();
    sorted_hands.iter().enumerate().map(|(i, hand)| hand.1 * (i as u32 + 1)).sum::<u32>()
}

fn main() {
    let input = include_str!("../../inputs/day07.in");
    // PT1
    let hands = input.lines().map(|line| parse_hand(line, Rules::NoJokers)).collect::<Vec<Hand>>();
    let pt1_winnings = calculate_winnings(&hands);
    println!("pt1: {}", pt1_winnings);
    // PT2
    let hands_joker_rule = input.lines().map(|line| parse_hand(line, Rules::Jokers)).collect::<Vec<Hand>>();
    let pt2_winnings = calculate_winnings(&hands_joker_rule);
    println!("pt2: {}", pt2_winnings);

}
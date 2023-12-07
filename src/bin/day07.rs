use itertools::Itertools;

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn is_five_of_a_kind(&self) -> bool {
        self.cards.iter().all(|card| *card == self.cards[0])
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards.iter().any(|card| self.cards.iter().filter(|c| **c == *card).count() == 4)
    }

    fn is_full_house(&self) -> bool {
        self.is_three_of_a_kind() && self.is_one_pair()
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.cards.iter().any(|card| self.cards.iter().filter(|c| **c == *card).count() == 3)
    }

    fn is_two_pairs(&self) -> bool {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();

        let mut count = 1;
        let mut pairs = 0;

        for i in 1..sorted_cards.len() {
            if sorted_cards[i] == sorted_cards[i - 1] {
                count += 1;
            } else {
                if count == 2 {
                    pairs += 1;
                }
                count = 1;
            }
        }

        pairs == 2 && count == 2
    }

    fn is_one_pair(&self) -> bool {
        self.cards.iter().any(|card| self.cards.iter().filter(|c| **c == *card).count() == 2)
    }

    fn worth(&self) -> [u32; 5] {
        let score = {
            if self.is_five_of_a_kind() {
                100000000
            } else if self.is_four_of_a_kind() {
                10000000
            } else if self.is_full_house() {
                1000000
            } else if self.is_three_of_a_kind() {
                100000
            } else if self.is_two_pairs() {
                10000
            } else if self.is_one_pair() {
                1000
            } else {
                0
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

fn map_card_to_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
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

fn parse_hand(line: &str) -> Hand {
    let mut hand_iter = line.split_whitespace();
    Hand {
        cards: hand_iter.next().unwrap().chars().map(|c| map_card_to_value(c)).collect(),
        bid: hand_iter.next().unwrap().parse::<u32>().unwrap(),
    }
}

fn main() {
    let input = include_str!("../../inputs/day07.in");
    let hands = input.lines().map(|line| parse_hand(line)).collect::<Vec<Hand>>();
    let sorted_hands = hands.iter()
        .map(|hand| (hand.worth(),  hand.bid))
        .sorted_by_key(|(worth, bid)| *worth)
        .collect::<Vec<_>>();
    let winnings = sorted_hands.iter().enumerate().map(|(i, hand)| hand.1 * (i as u32 + 1)).sum::<u32>();

    // dbg!(&sorted_hands[0..20]);
    println!("pt1: {}", winnings);
}
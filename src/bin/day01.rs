use itertools::Itertools;


fn concatenate_first_and_last_digit(line: &str, digits_to_look_for: &[&str]) -> u32 {
    let found_digits: Vec<(usize, &str)> = digits_to_look_for
        .iter()
        .flat_map(|digit| line.match_indices(digit).collect::<Vec<_>>())
        .sorted_by_key(|(idx, _)| *idx)
        .collect();

    let first = map_to_digit(found_digits.first().unwrap().1);
    let last = map_to_digit(found_digits.last().unwrap().1);
    format!("{}{}", first, last)
        .parse::<u32>()
        .unwrap()
}


fn map_to_digit(s: &str)  -> &str {
    match s {
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine"  | "9" => "9",
        _ => panic!("???")
    }
}

fn main() {
    let input = include_str!("../../inputs/day01.in");

    let digits_pt_1 = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let digits_pt_2 = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let pt1 = input
        .lines()
        .map(|line| concatenate_first_and_last_digit(line, &digits_pt_1))
        .sum::<u32>();

    let pt2 = input
        .lines()
        .map(|line| concatenate_first_and_last_digit(line, &digits_pt_2))
        .sum::<u32>();

    println!("pt1; {}", pt1);
    println!("pt2: {}", pt2);
}
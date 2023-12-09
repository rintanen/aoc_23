
enum Extrapolate {
    Forward,
    Backward,
}


fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn next_number(history: &Vec<i32>, extrapolate_direction: Extrapolate) -> i32 {
    let differences = collect_differences(history);
    let next = match extrapolate_direction {
        Extrapolate::Forward => extrapolate_forward(&differences),
        Extrapolate::Backward => extrapolate_backward(&differences),
    };
    next
}


fn extrapolate_forward(differences: &Vec<Vec<i32>>) -> i32 {
    let mut next = 0_i32;
    for difference in differences.iter().rev() {
        next = difference.last().unwrap() + next;
    }
    next
}

fn extrapolate_backward(differences: &Vec<Vec<i32>>) -> i32 {
    let mut next = 0_i32;
    for difference in differences.iter().rev() {
        next = difference.first().unwrap() - next;
    }
    next
}

fn collect_differences(history: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut diff = history.clone();
    let mut differences = vec![history.clone()];
    loop {
        diff = difference(diff);
        if diff.iter().all(|&d| d == 0) {
            break;
        }
        differences.push(diff.clone());
    }
    differences
}

fn difference(history: Vec<i32>) -> Vec<i32> {
    history.windows(2).map(|w| w[1] - w[0]).collect()
}


fn main() {
    let input = include_str!("../../inputs/day09.in");
    let histories: Vec<Vec<i32>> = input.lines().map(|line| parse_line(line)).collect();
    let sum_of_nexts = histories.iter().map(|history| next_number(history, Extrapolate::Forward)).sum::<i32>();
    println!("pt1: {}", sum_of_nexts);
    let sum_of_nexts = histories.iter().map(|history| next_number(history, Extrapolate::Backward)).sum::<i32>();
    println!("pt2: {}", sum_of_nexts);
}

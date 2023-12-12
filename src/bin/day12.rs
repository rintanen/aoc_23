

fn generate_combinations(s: &Vec<u8>, current: &mut Vec<u8>, index: usize) -> Vec<Vec<u8>> {
    if index == s.len() {
        return vec![current.clone()];
    }

    let mut combinations = Vec::new();
    let current_byte = s[index];

    match current_byte as char {
        '?' => {
            current.push(b'.');
            combinations.extend_from_slice(&generate_combinations(s, current, index + 1));
            current.pop();

            current.push(b'#');
            combinations.extend_from_slice(&generate_combinations(s, current, index + 1));
            current.pop();
        }
        _ => {
            current.push(current_byte);
            combinations.extend_from_slice(&generate_combinations(s, current, index + 1));
            current.pop();
        }
    }

    combinations
}


fn find_contiguous_periods(s: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut start_idx: Option<usize> = None;

    for (i, &byte) in s.iter().enumerate() {

        if byte == b'#' && start_idx.is_none() {
            // Start of a contiguous period
            start_idx = Some(i);
        } else if byte != b'#' && start_idx.is_some() {
            // End of a contiguous period
            let end_idx = i - 1;
            result.push((start_idx.unwrap(), end_idx));
            start_idx = None;
        }
    }

    // Check if there is a contiguous period at the end of the vector
    if let Some(start) = start_idx {
        result.push((start, s.len() - 1));
    }

    format_contiguous_periods(result)
}


fn format_contiguous_periods(periods: Vec<(usize, usize)>) -> Vec<u8> {
    let mut result = Vec::new();
    for (start, end) in periods {
        result.push((end - start + 1) as u8);
    }
    result
}



fn main() {
    let input = include_str!("../../inputs/day12.in");

    let mut contiguous_groups = vec![];
    let mut  arrangements = vec![];
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let arrangement = it.next().unwrap().bytes().collect::<Vec<u8>>();
        let contiguous_group = it.next().unwrap().split(',').map(|n| n.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        contiguous_groups.push(contiguous_group);
        arrangements.push(arrangement);
    }

    let mut sum = 0;
    for (arrangement, contiguous_periods) in arrangements.iter().zip(contiguous_groups.iter()) {
        let mut current = Vec::new();
        let all_combinations = generate_combinations(arrangement, &mut current, 0);
        let n_possible_arrangements = all_combinations.iter()
            .map(|arrangement| find_contiguous_periods(arrangement))
            .filter(|periods| periods == contiguous_periods)
            .count();
        sum += n_possible_arrangements;
    }
    println!("{}", sum);
}
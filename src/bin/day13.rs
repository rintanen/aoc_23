

fn find_horizontal_reflection(pattern: &Vec<&[u8]>) -> Option<usize> {
    for col in 0..pattern[0].len() {
        if pattern.iter().all(|row| horizontal_reflection_exists(row, col)) {
            return Some(col);
        }
    }
    None
}



fn find_vertical_reflection(pattern: &Vec<&[u8]>) -> Option<usize> {
    for row in 0..pattern.len() {
        if pattern.iter().all(|col| vertical_reflection_exists(&pattern, row)) {
            if row > pattern.len() / 2 {
                return Some(row-1);;
            }
            return Some(row);
        }
    }
    None
}

fn horizontal_reflection_exists(row: &[u8], index: usize) -> bool {
    let how_many_to_look_right = 2 * index + 1;
    let how_many_to_look_left = row.len() - index;

    if index < row.len() / 2 {
        row[0..=index].iter().rev().zip(row[index+1..=how_many_to_look_right].iter())
            .all(|(a, b)| a == b)
    } else if index > row.len() / 2 {
        let db_left = &row[index - how_many_to_look_left..index];
        let db_right = &row[index..];
        row[index - how_many_to_look_left..index].iter().rev().zip(row[index..].iter())
            .all(|(a, b)| a == b)
    } else {
        row[1..=index].iter().rev().zip(row[index+1..].iter()).all(|(a, b)| a == b) ||
            row[0..index].iter().rev().zip(row[index..row.len()-1].iter()).all(|(a, b)| a == b)
    }
}


fn vertical_reflection_exists(pattern: &Vec<&[u8]>, index: usize) -> bool {
    let how_many_to_look_up = 2 * index + 1;
    let how_many_to_look_down = pattern.len() - index;

    if index < pattern.len() / 2 {
        pattern[0..=index].iter().rev().zip(pattern[index+1..=how_many_to_look_up].iter())
            .all(|(a, b)| a == b)
    } else if index > pattern.len() / 2 {
        pattern[index - how_many_to_look_down..index].iter().rev().zip(pattern[index..].iter())
            .all(|(a, b)| a == b)
    } else {
        pattern[1..=index].iter().rev().zip(pattern[index+1..].iter()).all(|(a, b)| a == b) ||
        pattern[0..index].iter().rev().zip(pattern[index..pattern.len()-1].iter()).all(|(a, b)| a == b)
    }
}

fn summarize_patterns(patterns: &Vec<Vec<&[u8]>>) -> usize {
    let mut sum = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        if let Some(cols) = find_horizontal_reflection(pattern) {
            println!("Pattern {} has horizontal reflection between cols {}-{}", i, cols, cols+1);
            sum += cols + 1;
        }
        if let Some(rows) = find_vertical_reflection(pattern) {
            println!("Pattern {} has vertical reflection between rows {}-{}", i, rows, rows+1);
            sum += (rows + 1) * 100;
        }
        // } else {
        //     panic!("{}", format!("Pattern {} no reflection found", i));
        // }
    }
    sum
}


fn main() {
    let input = include_str!("../../inputs/day13.in");
    let patterns: Vec<&str> = input.split("\n\n").collect();
    // Map each chunk to a vector of byte arrays
    let patterns: Vec<Vec<&[u8]>> = patterns
        .iter()
        .map(|&chunk| chunk.lines().map(|line| line.as_bytes()).collect())
        .collect();

    // let horizontal = find_horizontal_reflection(&patterns[0]);
    // dbg!(horizontal);
    // let vertical = find_vertical_reflection(&patterns[0]);
    // dbg!(vertical);
    //
    // let horizontal = find_horizontal_reflection(&patterns[1]);
    // dbg!(horizontal);
    // let vertical = find_vertical_reflection(&patterns[1]);
    // dbg!(vertical);TT
    // // Find the horizontal reflection
    // let asdf = 9 as usize / 2;
    // println!("{}", asdf);
    let sum = summarize_patterns(&patterns);T
    dbg!(sum);


    /// TUTKI 94
}
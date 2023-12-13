




fn find_horizontal_reflection(pattern: &Vec<&[u8]>) -> Option<usize> {
    for col in 0..pattern[0].len() {
        if pattern.iter().all(|row| reflection_exists(row, col)) {
            return Some(col);
        }
    }
    None
}


fn reflection_exists(row: &[u8], index: usize) -> bool {
    if index < row.len() / 2 {
        todo!()
    } else if index > row.len() / 2 {
        todo!()
    } else {
        todo!()
    }
    let how_many_to_look_left = row.len() - index - 1;
    row[index..index-how_many_to_look_left] == row[index+1..row.len()-1]
}








fn main() {
    let input = include_str!("../../inputs/day13.in");
    let patterns: Vec<&str> = input.split("\n\n").collect();
    // Map each chunk to a vector of byte arrays
    let patterns: Vec<Vec<&[u8]>> = patterns
        .iter()
        .map(|&chunk| chunk.lines().map(|line| line.as_bytes()).collect())
        .collect();

    // Find the horizontal reflection
    let asdf = 9 as usize / 2;
    println!("{}", asdf);



}
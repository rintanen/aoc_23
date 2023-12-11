use itertools::Itertools;

fn empty_rows(input: &str) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if !line.contains('#') {
            empty_rows.push(i);
        }
    }
    empty_rows
}

fn empty_cols(input: &str) -> Vec<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut empty_cols = Vec::new();
    for i in 0..lines[0].len() {
        if !lines.iter().any(|l| l.chars().nth(i).unwrap() == '#')       {
            empty_cols.push(i);
        }
    }
    empty_cols
}


fn get_galaxies(input: &str, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                let row_adjust = empty_rows.iter().filter(|&r| *r < i).count();
                let col_adjust = empty_cols.iter().filter(|&c| *c < j).count();
                galaxies.push((i + row_adjust, j + col_adjust));
            }
        }
    }
    galaxies
}


fn manhattan_distance(first: (usize, usize), second: (usize, usize)) -> usize {
    (first.0 as isize - second.0 as isize).abs() as usize + (first.1 as isize - second.1 as isize).abs() as usize
}


fn main() {
    let input = include_str!("../../inputs/day11.in");
    let empty_rows = empty_rows(input);
    let empty_cols = empty_cols(input);
    // dbg!(&empty_rows);
    // dbg!(&empty_cols);
    let galaxies = get_galaxies(input, &empty_rows, &empty_cols);
    dbg!(&galaxies);

    let distance_between_galaxies = galaxies.iter()
        .permutations(2)
        .map(|pair| manhattan_distance(*pair[0], *pair[1]))
        .sum::<usize>();
    println!("Part 1: {}", distance_between_galaxies);

}
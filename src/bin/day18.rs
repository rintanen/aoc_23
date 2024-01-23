use itertools::Itertools;

fn shoelace(coords: &Vec<(i32, i32)>) -> f64 {
    let mut sum = 0.0;
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        sum += coords[i].0 as f64 * coords[j].1 as f64;
        sum -= coords[i].1 as f64 * coords[j].0 as f64;
    }
    sum.abs() / 2.0
}


fn main() {
    let input = include_str!("../../inputs/day18.in");

    let mut polygon: Vec<(i32, i32)> = Vec::new();
    polygon.push((0, 0));

    for line in input.lines() {
        for (direction, meters, _) in line.split_whitespace().tuples() {
            let delta = match direction {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (-1, 0),
                "D" => (1, 0),
                _ => unreachable!()
            };
            for _ in 0..meters.parse::<u32>().unwrap() {
                polygon.push(
                    (polygon.last().unwrap().0 + delta.0, polygon.last().unwrap().1 + delta.1)
                );
            }
        }
    }
    // Remove the last point, which is the same as the first
    polygon.pop();

    // println!("{:?}", coords);

    let area = shoelace(&polygon);
    println!("Area: {}", area);
    dbg!(polygon.len());
}
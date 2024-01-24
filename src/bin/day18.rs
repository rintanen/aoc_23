use std::mem::transmute;
use itertools::Itertools;

fn trapezoid(polygon: &Vec<(i32, i32)>) -> f32 {
    let mut sum = 0.0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let delta = (a.0-b.0).abs().max((a.1-b.1).abs());
        sum += (a.0 + b.0) as f32 * (a.1 - b.1) as f32 + delta as f32;
    }
    sum.abs() / 2.0 + 1.0
}


fn main() {
    let input = include_str!("../../inputs/day18.in");

    let mut polygon: Vec<(i32, i32)> = Vec::new();
    polygon.push((0, 0));

    for line in input.lines() {
        for (direction, meters, _) in line.split_whitespace().tuples() {
            let delta = match direction {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!()
            };
            let meters = meters.parse::<i32>().unwrap();
            polygon.push(
                (polygon.last().unwrap().0 + delta.0 * meters, polygon.last().unwrap().1 + delta.1 * meters)
            );
        }
    }
    // Remove the last point, which is the same as the first
    polygon.pop();

    let area = trapezoid(&polygon);
    dbg!(area);


}
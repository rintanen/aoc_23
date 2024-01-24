fn trapezoid(polygon: &Vec<(i32, i32)>) -> f64 {
    let mut sum = 0.0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let delta = (a.0 - b.0).abs().max((a.1 - b.1).abs());
        sum += (a.0 + b.0) as f64 * (a.1 - b.1) as f64 + delta as f64;
    }
    sum.abs() / 2.0 + 1.0
}

fn create_polygon(dig_pattern: Vec<(&str, i32)>) -> Vec<(i32, i32)> {
    let mut polygon: Vec<(i32, i32)> = Vec::new();
    polygon.push((0, 0));

    for (direction, meters) in dig_pattern {
        let delta = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!()
        };
        polygon.push(
            (polygon.last().unwrap().0 + delta.0 * meters, polygon.last().unwrap().1 + delta.1 * meters)
        );
    }
    // Remove the last point, which is the same as the first
    polygon.pop();
    polygon
}


fn main() {
    let input = include_str!("../../inputs/day18.in");

    let pattern_pt1: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap();
            let value = parts.next().unwrap();
            (direction, value.parse::<i32>().unwrap())
        })
        .collect();

    let polygon = create_polygon(pattern_pt1);

    let area_pt1 = trapezoid(&polygon);

    dbg!(area_pt1);

    let pattern_pt2: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().skip(2);
            let hex_code = parts.next().unwrap();
            let value = i32::from_str_radix(&hex_code[2..7], 16).unwrap();
            let direction = match &hex_code[7..=7] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => unreachable!()
            };
            (direction, value)
        })
        .collect();
    let polygon = create_polygon(pattern_pt2);
    let area_pt2 = trapezoid(&polygon);
    dbg!(area_pt2);
}
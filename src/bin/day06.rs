fn how_many_waits_beat_record(time: &u64, record: &u64) -> u64 {
    let mut waits_that_beat_record = 0;
    for wait in 0..*time {
        if distance_to_cover(wait, *time) > *record {
            waits_that_beat_record += 1;
        }
    }
    waits_that_beat_record
}

fn distance_to_cover(wait: u64,  time: u64) -> u64 {
    (time - wait) * wait
}


fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (root1, root2)
}


fn concat_numbers(numbers: &[u64]) -> u64 {
    numbers.iter().map(|n| n.to_string()).fold(String::new(), |acc, s| acc + &s).parse::<u64>().unwrap()
}


fn main() {
    let input = include_str!("../../inputs/day06.in").lines().collect::<Vec<_>>();
    let times = input[0].trim_start_matches("Time:").split_whitespace().filter(|w| !w.is_empty()).map(|w| w.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let records_to_beat = input[1].trim_start_matches("Distance:").split_whitespace().filter(|w| !w.is_empty()).map(|w| w.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let num_ways_to_beat = times.iter().zip(records_to_beat.iter())
        .map(|(time, record)| how_many_waits_beat_record(time, record))
        .product::<u64>();

    println!("pt1: {}", num_ways_to_beat);

    // pt 2
    let time = concat_numbers(&times);
    let record_to_beat = concat_numbers(&records_to_beat);

    // find roots of -1 * wait^2 + time * wait - record_to_beat
    let a = -1.0;
    let b = time as f64;
    let c = -1.0 * record_to_beat as f64;

    let roots = quadratic_roots(a, b, c);
    let result = roots.1.ceil() as u64 - roots.0.ceil() as u64;
    println!("pt2: {}", result);
}
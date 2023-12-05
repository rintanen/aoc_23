#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn find_range(&self, item: u32) -> Option<&Range> {
        self.ranges.iter().find(|range| range.contains(item))
    }

    fn map_source_to_destination(&self, item: u32) -> u32 {
        self.find_range(item).map(|range| range.map_to_destination(item)).unwrap_or(item)
    }
}

#[derive(Debug)]
struct Range {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

impl Range {
    fn contains(&self, item: u32) -> bool {
        item >= self.source_start && item < self.source_start + self.length
    }

    fn map_to_destination(&self, item: u32) -> u32 {
        self.destination_start + (item - self.source_start)
    }
}

fn parse_seeds(input: &str) -> Vec<u32> {
    input.trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn parse_map(section: &str) -> Map {
    let mut lines = section.lines().skip(1).collect::<Vec<_>>();
    let ranges = lines.iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let destination_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let source_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let length = line_iter.next().unwrap().parse::<u32>().unwrap();
            Range {source_start, destination_start, length }
        })
        .collect::<Vec<Range>>();
    Map { ranges }
}


fn map_seed_to_location(seed: u32, maps: &[Map]) -> u32 {
    let mut item = seed;
    for map in maps {
        item = map.map_source_to_destination(item);
    }
    item
}


// fn binary_search<F>(mut start: u32, mut end: u32, mut f: F) -> u32
// where
//     F: FnMut(u32) -> bool,
// {
//     while start < end {
//         let mid = start + (end - start) / 2;
//         if f(mid) {
//             end = mid;
//         } else {
//             start = mid + 1;
//         }
//     }
//     start
// }
// //
// //
// fn ternary_search<F>(mut start: u32, mut end: u32, mut f: F) -> u32
// where
//     F: FnMut(u32) -> bool,
// {
//     while start < end {
//         let mid1 = start + (end - start) / 3;
//         let mid2 = end - (end - start) / 3;
//         if f(mid1) {
//             end = mid1;
//         } else if f(mid2) {
//             start = mid1 + 1;
//             end = mid2;
//         } else {
//             start = mid2 + 1;
//         }
//     }
//     start
// }

fn binary_search_minimizer(maps: &[Map], low: u32, high: u32) -> u32 {
    const EPSILON: u32 = 1;

    let mut low = low;
    let mut high = high;

    while high - low > EPSILON {
        let mid = low + (high - low) / 2;

        // Evaluate the function at mid and mid+1
        let mid_value = map_seed_to_location(mid, maps);
        let mid_plus_one_value = map_seed_to_location(mid + 1, maps);

        if mid_value <= mid_plus_one_value {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}



fn main() {
    let input = include_str!("../../inputs/day05.in");
    let mut section_iter = input.split("\n\n");

    let seeds = parse_seeds(section_iter.next().unwrap());

    let maps = section_iter
        .map(|section| parse_map(section))
        .collect::<Vec<Map>>();
    // dbg!(&maps);

    let pt1 = seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, &maps))
        .min()
        .unwrap();

    println!("Part 1: {:?}", pt1);



    let seed_ranges = seeds.chunks(2).map(|s| (s[0], s[0] + s[1] - 1)).collect::<Vec<_>>();
    // let fars = seed_ranges.iter().map(|seed_range| ternary_search(seed_range.0, seed_range.1, |i| map_seed_to_location(i, &maps))).collect::<Vec<_>>();
    // let fars = seed_ranges.iter().map(|seed_range| binary_search_minimizer(&maps, seed_range.0, seed_range.1)).collect::<Vec<_>>();

    let low_bound = seed_ranges[0].0;
    let high_bound = seed_ranges[0].1;

    let result = binary_search_minimizer(&maps, low_bound, high_bound);

    dbg!(seed_ranges);
    println!("Part 2: {:?}", map_seed_to_location(79, &maps));
    println!("Part 2: {:?}", map_seed_to_location(82, &maps));
    dbg!(result);
    // dbg!(fars);
}
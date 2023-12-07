#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn find_range(&self, item: u64) -> Option<&Range> {
        self.ranges.iter().find(|range| range.contains(item))
    }

    fn map_source_to_destination(&self, item: u64) -> u64 {
        self.find_range(item).map(|range| range.map_to_destination(item)).unwrap_or(item)
    }
}

#[derive(Debug)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl Range {
    fn contains(&self, item: u64) -> bool {
        item >= self.source_start && item < self.source_start + self.length
    }

    fn map_to_destination(&self, item: u64) -> u64 {
        self.destination_start + (item - self.source_start)
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input.trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn parse_map(section: &str) -> Map {
    let lines = section.lines().skip(1).collect::<Vec<_>>();
    let ranges = lines.iter()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let destination_start = line_iter.next().unwrap().parse::<u64>().unwrap();
            let source_start = line_iter.next().unwrap().parse::<u64>().unwrap();
            let length = line_iter.next().unwrap().parse::<u64>().unwrap();
            Range {source_start, destination_start, length }
        })
        .collect::<Vec<Range>>();
    Map { ranges }
}


fn map_seed_to_location(seed: u64, maps: &[Map]) -> u64 {
    let mut item = seed;
    for map in maps {
        item = map.map_source_to_destination(item);
    }
    item
}

fn binary_search(maps: &[Map], (low, high): (u64, u64)) -> u64 {
    let mut low = low;
    let mut high = high;
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        let left = map_seed_to_location(mid - 1, maps);
        let right = map_seed_to_location(mid + 1, maps);
        if left <= right {
            high = mid;
        } else {
            low = mid;
        }
    }
    map_seed_to_location(low, maps)
}

fn main() {
    let input = include_str!("../../inputs/day05.in");
    let mut section_iter = input.split("\n\n");

    let seeds = parse_seeds(section_iter.next().unwrap());

    let maps = section_iter
        .map(|section| parse_map(section))
        .collect::<Vec<Map>>();

    let pt1 = seeds
        .iter()
        .map(|seed| map_seed_to_location(*seed, &maps))
        .min()
        .unwrap();
    println!("Part 1: {:?}", pt1);

    let seed_ranges = seeds.chunks(2).map(|s| (s[0], s[0] + s[1] - 1)).collect::<Vec<_>>();
    let pt2 = seed_ranges.iter()
        .map(|seed_range| binary_search(&maps, *seed_range))
        .min()
        .unwrap();
    println!("Part 2: {:?}", pt2);
}

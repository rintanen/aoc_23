#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

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
    fn new(source_start: u32, destination_start: u32, length: u32) -> Self {
        Self {
            source_start,
            destination_start,
            length,
        }
    }

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
            Range::new(source_start, destination_start, length)
        })
        .collect::<Vec<Range>>();
    Map { ranges }
}


fn main() {
    let input = include_str!("../../inputs/day05.in");
    let mut section_iter = input.split("\n\n");

    let seeds = parse_seeds(section_iter.next().unwrap());

    let maps = section_iter
        .map(|section| parse_map(section))
        .collect::<Vec<Map>>();
    dbg!(maps);




}
use std::collections::{VecDeque, HashSet};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct LightPropagation {
    direction: Direction,
    position: (usize, usize),
}

impl LightPropagation {
    fn propagate_and_split(&mut self, grid: &Vec<Vec<char>>) -> Option<LightPropagation> {

        let new_direction = self.next_direction(grid[self.position.0][self.position.1]);

        self.position = match new_direction {
            Direction::Right => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0, self.position.1 - 1),
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Down => (self.position.0 + 1, self.position.1),
        };

        self.direction = new_direction;
    }


    fn next_direction(&self, tile: char) -> Direction {
        match (&self.direction, tile) {
            (_, '.') => *self.direction,
            (Direction::Right, '|') | (Direction::Right, '\\') => Direction::Down,
            (Direction::Right, '/') => Direction::Up,
            (Direction::Right, '-') => Direction::Right,

            (Direction::Left, '|') | (Direction::Left, '\\') => Direction::Down,
            (Direction::Left, '/') => Direction::Up,
            (Direction::Left, '-') => Direction::Left,

            (Direction::Up, '|') => Direction::Up,
            (Direction::Up, '\\') => Direction::Left,
            (Direction::Up, '/') | (Direction::Up, '-') => Direction::Right,

            (Direction::Down, '|') | (Direction::Down, '\\') => Direction::Down,
            (Direction::Down, '/') => Direction::Left,
            (Direction::Down, '-') => Direction::Right,
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/day16.in");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let initial_light = LightPropagation {
        direction: Direction::Right,
        position: (0, 0),
    };

    let mut lights: VecDeque<LightPropagation> = VecDeque::new();

    lights.push_back(initial_light);

    let mut illuminated: HashSet<(usize, usize)> = HashSet::new();

    while let Some(mut light) = lights.pop_front() {
        loop {
            if let Some(splitted_light) = light.propagate_and_split(&grid) {
                lights.push_back(splitted_light);
            }
            if light.position.0 >= grid.len() || light.position.1 >= grid[0].len() {
                continue;
            }
            if illuminated.contains(&light.position) {
                continue;
            }
            illuminated.insert(light.position);
            // break loop if light hits wall
            // if grid[light.position.0][light.position.1] == '#' {
            //     break;
            // }
        }
    }


}
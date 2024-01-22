use std::collections::{VecDeque, HashSet};
use rayon::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Propagation {
    Split(Light),
    Continue,
    Stop,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Light {
    direction: Direction,
    position: (usize, usize),
}

impl Light {
    fn new(direction: Direction, position: (usize, usize)) -> Self {
        Self { direction, position }
    }

    fn propagate(&mut self, grid: &Vec<Vec<char>>) -> Propagation {
        let tile = grid[self.position.0][self.position.1];
        let propagation: Propagation;
        match (self.direction, tile) {
            (_, '.') => {propagation = Propagation::Continue;},
            (Direction::Right, '|') | (Direction::Left, '|') => {
                propagation = Propagation::Split(Light::new(Direction::Up, self.position));
            },
            (Direction::Up, '-') | (Direction::Down, '-') => {
                propagation = Propagation::Split(Light::new(Direction::Left, self.position));
            },
            _ => {
                propagation = Propagation::Continue;
            },
        }

        self.direction = self.next_direction(tile);

        if light_hits_wall(self.position, self.direction, grid) {
            return Propagation::Stop;
        }

        // update location
        match self.direction {
            Direction::Right => self.position.1 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
        }
        propagation
    }

    fn next_direction(&self, tile: char) -> Direction {
        match (&self.direction, tile) {
            (_, '.') => self.direction,
            (Direction::Right, '|') | (Direction::Right, '\\') => Direction::Down,
            (Direction::Right, '/') => Direction::Up,
            (Direction::Right, '-') => Direction::Right,

            (Direction::Left, '|') | (Direction::Left, '/') => Direction::Down,
            (Direction::Left, '\\') => Direction::Up,
            (Direction::Left, '-') => Direction::Left,

            (Direction::Up, '|') => Direction::Up,
            (Direction::Up, '\\') => Direction::Left,
            (Direction::Up, '/') | (Direction::Up, '-') => Direction::Right,

            (Direction::Down, '|')  => Direction::Down,
            (Direction::Down, '/') => Direction::Left,
            (Direction::Down, '-') | (Direction::Down, '\\') => Direction::Right,
            _ => panic!("???"),
        }
    }
}

fn light_hits_wall(position: (usize, usize), direction: Direction, grid: &Vec<Vec<char>>) -> bool {
    match direction {
        Direction::Right => position.1 == grid[0].len() - 1,
        Direction::Left => position.1 == 0,
        Direction::Up => position.0 == 0,
        Direction::Down => position.0 == grid.len() - 1,
    }
}


fn fill_space_with_light(grid: &Vec<Vec<char>>, light: Light) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(light);

    let mut visited: HashSet<Light> = HashSet::new();
    while let Some(mut light) = queue.pop_front() {
        loop {
            if visited.contains(&light) {
                break;
            }
            visited.insert(light.clone());
            match light.propagate(&grid) {
                Propagation::Split(splitted_light) => {
                    queue.push_back(splitted_light);
                    continue
                },
                Propagation::Continue => continue,
                Propagation::Stop => break,
            }
        }
    }
    let visited: HashSet<(usize, usize)> = visited.iter().map(|light| light.position).collect();
    visited.len()
}


fn main() {
    let input = include_str!("../../inputs/day16.in");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let initial_light = Light::new(Direction::Right, (0, 0));

    let energized = fill_space_with_light(&grid, initial_light);

    dbg!(energized);

    let mut possible_starting_lights: Vec<Light> = vec![Light::new(Direction::Right, (0, 0)),
                                                        Light::new(Direction::Down, (0, 0)),

                                                        Light::new(Direction::Left, (0, grid[0].len() - 1)),
                                                        Light::new(Direction::Down, (0, grid[0].len() - 1)),

                                                        Light::new(Direction::Up, (grid.len() - 1, 0)),
                                                        Light::new(Direction::Right, (grid.len() - 1, 0)),

                                                        Light::new(Direction::Down, (grid.len() - 1, grid[0].len() - 1)),
                                                        Light::new(Direction::Left, (grid.len() - 1, grid[0].len() - 1)),
                                                        ];
    // add possible light from leftern most and righternmost column except corners
    for i in 1..grid.len() - 1 {
        possible_starting_lights.push(Light::new(Direction::Right, (i, 0)));
        possible_starting_lights.push(Light::new(Direction::Left, (i, grid[0].len() - 1)));
    }

    // add possible light from top and bottom row except corners
    for i in 1..grid[0].len() - 1 {
        possible_starting_lights.push(Light::new(Direction::Down, (0, i)));
        possible_starting_lights.push(Light::new(Direction::Up, (grid.len() - 1, i)));
    }

    let pt2 = possible_starting_lights.par_iter().map(|light| fill_space_with_light(&grid, *light)).max().unwrap();
    dbg!(pt2);

}
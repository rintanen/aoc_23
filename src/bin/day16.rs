use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy)]
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

struct Light {
    direction: Direction,
    position: (usize, usize),
}

impl Light {
    fn new(direction: Direction, position: (usize, usize)) -> Self {
        Self { direction, position }
    }

    fn propagate(&mut self, grid: &Vec<Vec<char>>) -> Propagation {
        self.direction = self.next_direction(grid[self.position.0][self.position.1]);

        match self.direction {
            Direction::Right => {
                if self.position.1 == grid[0].len() {
                    Propagation::Stop
                } else {
                    self.position.1 += 1;
                    Propagation::Continue
                }
            },
            Direction::Left => {
                if self.position.1 == 0 {
                    Propagation::Stop
                } else {
                    self.position.1 -= 1;
                    Propagation::Continue
                }
            },
            Direction::Up => {
                if self.position.0 == 0 {
                    Propagation::Stop
                } else {
                    self.position.0 -= 1;
                    Propagation::Continue
                }
            },
            Direction::Down => {
                if self.position.0 == grid.len() {
                    Propagation::Stop
                } else {
                    self.position.0 += 1;
                    Propagation::Continue
                }
            },
        }
    }

    /*
    Handlays sille kun tulee splitattu valo pitää tehdä mistä se palautetaan tuonne
    */

    fn split(&mut self, grid: &Vec<Vec<char>>) -> Propagation {
        match self.direction {
            Direction::Right => {
                if self.position.1 == grid[0].len() {
                    Propagation::Stop
                } else {
                    self.position.1 += 1;
                    Propagation::Continue
                }
            },
            Direction::Left => {
                if self.position.1 == 0 {
                    Propagation::Stop
                } else {
                    self.position.1 -= 1;
                    Propagation::Continue
                }
            },
            Direction::Up => {
                if self.position.0 == 0 {
                    Propagation::Stop
                } else {
                    self.position.0 -= 1;
                    Propagation::Continue
                }
            },
            Direction::Down => {
                if self.position.0 == grid.len() {
                    Propagation::Stop
                } else {
                    self.position.0 += 1;
                    Propagation::Continue
                }
            },
        }
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

fn main() {
    let input = include_str!("../../inputs/day16.in");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let initial_light = Light::new(Direction::Right, (0, 0));

    let mut queue: VecDeque<Light> = VecDeque::new();

    queue.push_back(initial_light);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(mut light) = queue.pop_front() {
        loop {
            match light.propagate(&grid) {
                Propagation::Split(splitted_light) => queue.push_back(splitted_light),
                Propagation::Continue => continue,
                Propagation::Stop => break,
            }
            visited.insert(light.position);
        }
    }
}
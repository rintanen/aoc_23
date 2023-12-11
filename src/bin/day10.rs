use std::collections::HashSet;

struct Connection {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Connection {
    fn new(c: &u8) -> Connection {
        match c {
            b'-' => Connection { up: false, down: false, left: true, right: true },
            b'|' => Connection { up: true, down: true, left: false, right: false },
            b'J' => Connection { up: true, down: false, left: true, right: false },
            b'L' => Connection { up: true, down: false, left: false, right: true },
            b'7' => Connection { up: false, down: true, left: true, right: false },
            b'F' => Connection { up: false, down: true, left: false, right: true },
            b'S' => Connection { up: true, down: true, left: true, right: true },
            _ => Connection { up: false, down: false, left: false, right: false },
        }
    }
}

fn map_main_loop(grid: &Vec<Vec<Connection>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut main_loop: HashSet<(usize, usize)> = HashSet::new();
    let mut loc = start;
    let mut prev = start;
    let mut temp;
    loop {
        temp = loc;
        loc = next_possible_loc(&grid, &loc.0, &loc.1, &prev);
        main_loop.insert(loc);
        prev = temp;
        if loc == start {
            break;
        }
    }
    main_loop
}

fn next_possible_loc(grid: &Vec<Vec<Connection>>, y: &usize, x: &usize, previous: &(usize, usize)) -> (usize, usize) {
    let up = (y.checked_sub(1).unwrap_or(0), *x);
    let down = (y + 1, *x);
    let left = (*y, x.checked_sub(1).unwrap_or(0));
    let right = (*y, x + 1);
    if grid[*y][*x].right && &right != previous && right.1 <= grid[0].len() {
        right
    } else if grid[*y][*x].left && &left != previous && x.checked_sub(1).is_some() {
        left
    } else if grid[*y][*x].up && &up != previous && y.checked_sub(1).is_some() {
        up
    } else if grid[*y][*x].down && &down != previous && down.0 <= grid.len() {
        down
    } else {
        panic!("No possible next location found!");
    }
}

fn enclosed_tiles(grid: &Vec<Vec<Connection>>, main_loop: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;
    for row in 1..grid.len() - 1 {
        let mut enclosed = false;
        for col in 1..grid[0].len() - 1 {
            if !main_loop.contains(&(row, col)) {
                count += enclosed as usize;
            } else if grid[row][col].up {
                enclosed = !enclosed;
            }
        }
    }
    count
}

fn main() {
    let input = include_str!("../../inputs/day10.in");
    let start = input.find('S').unwrap();
    let cols = input.find('\n').unwrap();
    let y = start / cols;
    let x = start % cols - y;
    let start = (y, x);

    let grid = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| Connection::new(c)).collect::<Vec<_>>())
        .collect::<Vec<Vec<Connection>>>();

    let main_loop = map_main_loop(&grid, start);
    println!("Furthest way from start {}", main_loop.len() / 2);

    let p2 = enclosed_tiles(&grid, &main_loop);
    println!("Number of points inside {}", p2 - 1);
}
use std::collections::HashSet;

#[derive(Debug)]
struct Grid <'a> {
    arr: &'a [u8],
    cols: usize,
    rows: usize,
}

impl<'a> Grid<'a> {
    fn new(arr: &'a [u8]) -> Grid {
        let cols = arr.iter().position(|&b| b == 10).unwrap();
        let rows = arr.len() / cols;
        Grid { arr, cols, rows }
    }

    fn get(&self, x: &usize, y: &usize) -> Option<&'a u8> {
        let index_1d = x + self.cols * y + y;
        // add row's index to the 2d->1d grid mapping index
        // because there is newline character between each row
        if index_1d < self.arr.len() {
            let res = Some(&self.arr[index_1d]);
            res
        } else {
            None
        }
    }

    fn find(&self, target: u8) ->  Option<(usize, usize)> {
        let index_1d = self.arr.iter().position(|c| c == &target);
        if let Some(i) = index_1d {
            let y = i / self.cols;
            let x = i % self.cols - y;
            Some((x, y))
        } else {
            None
        }
    }
}

fn travel_pipe(grid: &Grid, start: (usize, usize)) -> usize {
    let mut distance = 0;
    let mut loc = start;
    let mut prev = start;
    let mut temp;
    loop {
        temp = loc;
        loc = next_possible_loc(grid, &loc.0, &loc.1, &prev);
        prev = temp;
        distance += 1;
        if loc == start {
            break;
        }
    }
    distance
}

fn try_to_find_way_out(grid: &Grid, start: (usize, usize)) -> usize {
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut temp;
    /*
    liiku niin kauan sallitusti kunnes ei enää pysty liikkumaan tai jos pääsee reunalle
    */
    loop {
        if !points.insert(start) {

            ;
        }
        temp = loc;
        loc = next_possible_loc(grid, &loc.0, &loc.1, &prev);
        prev = temp;
        distance += 1;
        if loc == start {
            break;
        }
    }
}


fn next_possible_loc(grid: &Grid, x: &usize, y: &usize, previous: &(usize, usize)) -> (usize, usize) {
    let left = (x.checked_sub(1).unwrap_or(0), *y);
    let right = (x + 1, *y);
    let up = (*x, y.checked_sub(1).unwrap_or(0));
    let down = (*x, y + 1);
    
    let allowed_right = [
        (b'-', b'-'), (b'-', b'J'), (b'-', b'7'), (b'-', b'S'),
        (b'L', b'-'), (b'L', b'J'), (b'L', b'7'), (b'L', b'S'),
        (b'F', b'-'), (b'F', b'J'), (b'F', b'7'), (b'F', b'S'),
        (b'S', b'-'), (b'S', b'J'), (b'S', b'7'),
    ];
    let allowed_left = [
        (b'-', b'-'), (b'-', b'L'), (b'-', b'F'), (b'-', b'S'),
        (b'J', b'-'), (b'J', b'L'), (b'J', b'F'), (b'J', b'S'),
        (b'7', b'-'), (b'7', b'L'), (b'7', b'F'), (b'7', b'S'),
        (b'S', b'-'), (b'S', b'L'), (b'S', b'F'),
    ];
    let allowed_up = [
        (b'|', b'|'), (b'|', b'7'), (b'|', b'F'), (b'|', b'S'),
        (b'L', b'|'), (b'L', b'7'), (b'L', b'F'), (b'L', b'S'),
        (b'J', b'|'), (b'J', b'7'), (b'J', b'F'), (b'J', b'S'),
        (b'S', b'|'), (b'S', b'7'), (b'S', b'F'), (b'S', b'L'),
    ];
    let allowed_down = [
        (b'|', b'|'), (b'|', b'L'), (b'|', b'J'), (b'|', b'F'),
        (b'7', b'|'), (b'7', b'L'), (b'7', b'J'), (b'7', b'F'),
        (b'F', b'|'), (b'F', b'L'), (b'F', b'J'), (b'F', b'7'),
        (b'S', b'|'), (b'S', b'L'), (b'S', b'J'), (b'S', b'F'),
    ];
    // println!("Current location: {:?}", (x, y));

    // dbg!(*grid.get(x, y).unwrap() as char , *grid.get(&(x + 1), y).unwrap() as char);
    // dbg!(*grid.get(x, y).unwrap() as char , *grid.get(&(x - 1), y).unwrap() as char);
    // dbg!(*grid.get(x, y).unwrap() as char , *grid.get(x, &(y + 1)).unwrap() as char);
    // dbg!(*grid.get(x, y).unwrap() as char , *grid.get(x, &(y - 1)).unwrap() as char);


    if allowed_right.contains(&(*grid.get(x, y).unwrap(), *grid.get(&right.0, &right.1).unwrap())) && &right != previous && right.0 <= grid.cols {
        right
    } else if allowed_left.contains(&(*grid.get(x, y).unwrap(), *grid.get(&left.0, &left.1).unwrap())) && &left != previous && x.checked_sub(1).is_some() {
        left
    } else if allowed_up.contains(&(*grid.get(x, y).unwrap(), *grid.get(&up.0, &up.1).unwrap())) && &up != previous && y.checked_sub(1).is_some() {
        up
    } else if allowed_down.contains(&(*grid.get(x, y).unwrap(), *grid.get(&down.0, &down.1).unwrap())) && &down != previous && down.1 <= grid.rows {
        down
    } else {
        panic!("No possible next location found!");
    }
}



fn main() {
    let input = include_str!("../../inputs/day10.in");
    let grid = Grid::new(input.as_bytes());
    let start = grid.find(b'S').unwrap();

    let length = travel_pipe(&grid, start);

    // pt1 toimii tarvii vain handlays jos menee reunoille
    println!("Length of the pipe: {}", length);
    println!("Furthest way from start {}", length / 2);

}
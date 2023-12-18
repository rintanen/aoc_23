use std::collections::HashMap;

fn tilt(plane: &mut Vec<Vec<u8>>) {
    let mut done = false;
    while !done {
        done = true;
        for r in 0..plane.len() - 1 {
            for c in 0..plane[0].len() {
                if plane[r+1][c] == b'O' && plane[r][c] == b'.' {
                    plane[r][c] = b'O';
                    plane[r+1][c] = b'.';
                    done = false;
                }
            }
        }
    }
}

fn rotate(plane: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut newmap = vec![vec![0; plane.len()]; plane[0].len()];
    for r in 0..plane.len() {
        for c in 0..plane[0].len() {
            newmap[c][plane.len() - 1 - r] = plane[r][c];
        }
    }
    newmap
}

fn calculate_load(plane: &Vec<Vec<u8>>) -> usize {
    (0..plane.len())
        .map(|r| (0..plane[0].len())
            .filter(|&c| plane[r][c] == b'O')
            .map(|_| plane.len() - r)
            .sum::<usize>()
        )
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day14.in");
    let mut plane = input.split('\n').map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
    let p1 = {
        let mut map = plane.clone();
        tilt(&mut map);
        calculate_load(&map)
    };
    dbg!(p1);

    let mut seen_states = HashMap::new();
    for i in 1..1000000000 {
        for _ in 0..4 {
            tilt(&mut plane);
            plane = rotate(&plane);
        }
        if let Some(seen_at) = seen_states.insert(plane.clone(), i) {
            if (1000000000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }
    let p2 = calculate_load(&plane);
    dbg!(p2);
}

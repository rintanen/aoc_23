use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Balls {
    reds: usize,
    greens: usize,
    blues: usize
}

impl PartialOrd for Balls {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.reds <= other.reds && self.greens <= other.greens && self.blues <= other.blues {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Balls {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn remember_max_balls_shown(line: &str) -> Balls {
    let reds = max_balls_shown(line, "red");
    let greens = max_balls_shown(line, "green");
    let blues = max_balls_shown(line, "blue");
    Balls {reds, greens, blues}
}

fn max_balls_shown(line: &str, ball_color: &str) -> usize {
    line
        .match_indices(ball_color)
        .map(|(idx, _)| line[idx-3..idx-1].trim().parse::<usize>().unwrap())
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../../inputs/day02.in");

    let known_balls = Balls {reds: 12, greens: 13, blues: 14};

    let pt1 = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)|
            if remember_max_balls_shown(line) <= known_balls {
                Some(idx + 1)
            } else {
                None
        })
        .sum::<usize>();
    dbg!(pt1);

    let pt2 = input
        .lines()
        .map(|line| {
            let balls = remember_max_balls_shown(line);
            balls.reds * balls.greens * balls.blues
        })
        .sum::<usize>();
    dbg!(pt2);
}
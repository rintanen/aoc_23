use itertools::iproduct;

fn positions_taken(loc: (usize, usize), digits: usize) -> Vec<(usize, usize)> {
    iproduct!(
            loc.0..=loc.0 + digits - 1,
            std::iter::once(loc.1)
        ).collect::<Vec<_>>()
}

fn main() {
    dbg!(positions_taken((0, 0), 3));
}

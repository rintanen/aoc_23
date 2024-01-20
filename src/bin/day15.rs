use indexmap::IndexMap;

fn hash(word: &[u8]) -> usize {
    word.iter().fold(0, |hash, &w| (hash + w as usize) * 17 % 256)
}


fn main() {
    let input = include_str!("../../inputs/day15.in");
    let pt1 = input.split(',').map(|word| hash(word.as_bytes())).sum::<usize>();
    dbg!(pt1);

    let mut boxes: Vec<IndexMap<&str, usize>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(IndexMap::new());
    }

    for word in input.split(',') {
        let index = word.find(|c| c == '-' || c == '=').unwrap();
        let key = &word[..index];
        let box_id = hash(key.as_bytes());
        match &word[index..=index] {
            "-" => {
                boxes[box_id].shift_remove(key);
            },
            "=" => {
                let value = word[index+1..].parse::<usize>().unwrap();
                boxes[box_id].insert(key, value);
            },
            _ => panic!("Unexpected character??"),
        }
    }

    let pt2 = boxes
        .iter()
        .enumerate()
        .map(|(i_box, box_)| {
            box_
                .values()
                .enumerate()
                .map(|(position, &value)| (i_box + 1) * (position + 1) * value)
                .sum::<usize>()
        })
        .sum::<usize>();

    dbg!(pt2);
}
use std::cmp::Ordering;

fn main() {
    println!("Day 13");

    let input = include_str!("../input/part1.txt");
    // '10' prevents from comparing byte by byte, using 'A' instead which is also larger than 9 when comparing bytes
    let input = input.replace("10", "A");
    let pairs = input.split("\n\n").collect::<Vec<_>>();

    let sum_of_correct_pair_indices: usize = pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| {
            let (lh, rh) = pair.split_once('\n').unwrap();
            match compare_packets(lh.as_bytes(), rh.as_bytes()) {
                Ordering::Less => idx + 1,
                _ => 0,
            }
        })
        .sum();

    println!("part 1: {}", sum_of_correct_pair_indices);

    let mut packets = input.lines().filter(|l| !l.is_empty()).chain(vec!["[[2]]", "[[6]]"]).collect::<Vec<_>>();
    packets.sort_by(|lh, rh| compare_packets(lh.as_bytes(), rh.as_bytes()));

    let decoder_key = packets
        .iter()
        .enumerate()
        .map(|(idx, packet)| match *packet {
            p if p == "[[2]]" || p == "[[6]]" => idx + 1,
            _ => 0,
        })
        .filter(|key| *key != 0)
        .reduce(|a, b| a * b)
        .unwrap();
    println!("part 2: {}", decoder_key);
}

fn compare_packets(lh: &[u8], rh: &[u8]) -> Ordering {
    match (lh[0], rh[0]) {
        (l, r) if l == r => compare_packets(&lh[1..], &rh[1..]), // items are equal, move to next position
        (b']', _) => Ordering::Less,                             // r can not be ']', so left side ran out of items
        (_, b']') => Ordering::Greater,                          // l can not be ']', so right side ran out of items
        (b'[', r) => compare_packets(&lh[1..], &[&[r, b']'], &rh[1..]].concat()), // wrap right-hand item into array
        (l, b'[') => compare_packets(&[&[l, b']'], &lh[1..]].concat(), &rh[1..]), // wrap left-hand item into array
        (l, r) => l.cmp(&r), // none of the above matches, so we're left with numbers
    }
}

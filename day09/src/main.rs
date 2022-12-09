use std::{collections::HashSet, iter};

fn main() {
    println!("Day 9");

    let input = include_str!("../input/part1.txt");

    print!("Part 1 - nr of positions: {}\n", rope_simulation(input, 2));
    print!("Part 2 - nr of positions: {}\n", rope_simulation(input, 10));
}

fn rope_simulation(input: &str, rope_length: usize) -> usize {
    let mut visited_positions_tail: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    let mut current_rope_positions: Vec<(isize, isize)> =
        iter::repeat((0, 0)).take(rope_length).collect();

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<usize>().unwrap();

        for _ in 0..amount {
            current_rope_positions[0] = match dir {
                "U" => (current_rope_positions[0].0, current_rope_positions[0].1 + 1),
                "D" => (current_rope_positions[0].0, current_rope_positions[0].1 - 1),
                "L" => (current_rope_positions[0].0 - 1, current_rope_positions[0].1),
                "R" => (current_rope_positions[0].0 + 1, current_rope_positions[0].1),
                _ => {
                    panic!("unknown direction head")
                }
            };

            for i in 1..rope_length {
                let diff: (isize, isize) = (
                    current_rope_positions[i - 1].0 - current_rope_positions[i].0,
                    current_rope_positions[i - 1].1 - current_rope_positions[i].1,
                );

                let new_pos;

                if (-1..=1).contains(&diff.0) && (-1..=1).contains(&diff.1) {
                    new_pos = current_rope_positions[i];
                } else {
                    new_pos = (
                        current_rope_positions[i].0 + sanitize_diff(diff.0),
                        current_rope_positions[i].1 + sanitize_diff(diff.1),
                    );
                }

                current_rope_positions[i] = new_pos;
            }

            visited_positions_tail.insert(current_rope_positions[rope_length - 1]);
        }
    }

    visited_positions_tail.len()
}

fn sanitize_diff(value: isize) -> isize {
    value.max(-1).min(1)
}

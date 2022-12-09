use std::collections::HashMap;

fn main() {
    println!("Day 9");

    let input = include_str!("../input/part1.txt");

    print!(
        "Part 1 - positions visited: {}\n",
        rope_simulation(input, 2)
    );
    print!(
        "Part 2 - positions visited: {}\n",
        rope_simulation(input, 10)
    );
}

fn rope_simulation(input: &str, rope_length: usize) -> usize {
    let mut visited_positions_tail: HashMap<(isize, isize), usize> = HashMap::new();

    let mut current_rope_positions: Vec<(isize, isize)> = Vec::new();
    for _ in 0..rope_length {
        current_rope_positions.push((0, 0));
    }
    visited_positions_tail.insert((0, 0), 1);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<usize>().unwrap();

        for _ in 0..amount {
            let rope_head = current_rope_positions[0];
            let new_head_pos = match dir {
                "U" => (rope_head.0, rope_head.1 + 1),
                "D" => (rope_head.0, rope_head.1 - 1),
                "L" => (rope_head.0 - 1, rope_head.1),
                "R" => (rope_head.0 + 1, rope_head.1),
                _ => {
                    panic!("unknown direction head")
                }
            };
            current_rope_positions[0] = new_head_pos;

            for i in 1..rope_length {
                let new_pos;
                let diff: (isize, isize) = (
                    current_rope_positions[i - 1].0 - current_rope_positions[i].0,
                    current_rope_positions[i - 1].1 - current_rope_positions[i].1,
                );

                if (-1..=1).contains(&diff.0) && (-1..=1).contains(&diff.1) {
                    new_pos = current_rope_positions[i];
                } else {
                    new_pos = (
                        match diff.0 {
                            -2 => current_rope_positions[i].0 - 1,
                            -1 => current_rope_positions[i].0 - 1,
                            0 => current_rope_positions[i].0,
                            1 => current_rope_positions[i].0 + 1,
                            2 => current_rope_positions[i].0 + 1,
                            _ => panic!("unhandled value"),
                        },
                        match diff.1 {
                            -2 => current_rope_positions[i].1 - 1,
                            -1 => current_rope_positions[i].1 - 1,
                            0 => current_rope_positions[i].1,
                            1 => current_rope_positions[i].1 + 1,
                            2 => current_rope_positions[i].1 + 1,
                            _ => panic!("unhandled value"),
                        },
                    );
                }

                current_rope_positions[i] = new_pos;
            }

            visited_positions_tail
                .entry(current_rope_positions[rope_length - 1])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    visited_positions_tail.keys().count()
}

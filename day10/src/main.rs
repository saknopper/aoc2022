use std::collections::HashSet;

fn main() {
    println!("Day 10");

    let input = include_str!("../input/part1.txt");

    let signal_strength_check_at_cycles: HashSet<usize> =
        HashSet::from([20, 60, 100, 140, 180, 220]);

    let mut x_register: isize = 1;

    let mut cycle_register_values: Vec<isize> = Vec::new();

    for line in input.lines() {
        let mut instruction = line.split_ascii_whitespace();
        let operation = instruction.next().unwrap();
        let amount = match operation {
            "addx" => instruction.next().unwrap().parse::<isize>().unwrap(),
            _ => 0,
        };

        match operation {
            "addx" => {
                cycle_register_values.push(x_register);
                cycle_register_values.push(x_register);
            }
            "noop" => {
                cycle_register_values.push(x_register);
            }
            _ => panic!("unhandled operation"),
        }

        x_register += amount;
    }

    let sum: isize = signal_strength_check_at_cycles
        .iter()
        .map(|c| cycle_register_values[(*c) - 1] * (*c as isize))
        .sum();

    print!("Part 1 - {}\n", sum);
}

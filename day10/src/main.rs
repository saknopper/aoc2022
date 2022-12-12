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

    println!("Part 1 - {}", sum);

    let crt = cycle_register_values
        .iter()
        .enumerate()
        .map(|v| {
            let row_cycle = v.0 as isize % 40;
            if (v.1 - 1..=v.1 + 1).contains(&row_cycle) {
                return '#';
            }
            return '.';
        })
        .collect::<Vec<char>>();

    println!("Part 2");
    crt.chunks(40)
        .for_each(|crt_line| println!("{:?}", crt_line.iter().collect::<String>()));
}

fn main() {
    println!("Day 1");

    let input = include_str!("../input/part1.txt");

    let mut calories: Vec<u64> = Vec::new();
    for group in input.split("\n\n") {
        calories.push(group.lines().map(|l| l.parse::<u64>().unwrap()).sum());
    }

    calories.sort();

    print!("part 1: max calories: {}\n", calories.last().unwrap());

    let sum: u64 = calories.iter().rev().take(3).sum();
    print!("part 2: sum of 3 max calories: {}\n", sum);
}

fn main() {
    println!("Day 1");

    let input = include_str!("../input/part1.txt");

    let mut calories: Vec<u64> = Vec::new();
    calories.push(0);
    let mut index: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            index += 1;
            calories.push(0);
            continue;
        }

        calories[index] += line.parse::<u64>().unwrap();
    }

    calories.sort();

    print!("part 1: max calories: {}\n", calories.last().unwrap());

    let sum: u64 = calories.iter().rev().take(3).sum();
    print!("part 2: sum of 3 max calories: {}\n", sum);
}

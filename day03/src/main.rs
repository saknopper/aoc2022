fn main() {
    println!("Day 3");

    let input = include_str!("../input/part1.txt");

    let possible_items = ('a' as u8..'z' as u8 + 1)
        .chain('A' as u8..'Z' as u8 + 1)
        .map(|i| i as char)
        .collect::<Vec<_>>();

    let mut total_score: u64 = 0;
    for line in input.lines() {
        let compartments = line.split_at(line.len() / 2);

        for (i, item) in possible_items.iter().enumerate() {
            if compartments.0.contains(*item) && compartments.1.contains(*item) {
                total_score += i as u64 + 1;
                break;
            }
        }
    }

    print!("Part 1 - score is: {}\n", total_score);

    total_score = 0;
    let lines: Vec<&str> = input.lines().into_iter().collect();
    for group in lines.chunks(3) {
        for (i, item) in possible_items.iter().enumerate() {
            if group.iter().all(|r| r.contains(*item)) {
                total_score += i as u64 + 1;
                break;
            }
        }
    }

    print!("Part 1 - score is: {}\n", total_score);
}

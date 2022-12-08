use std::collections::HashMap;

fn main() {
    println!("Day 2");

    let input = include_str!("../input/part1.txt");

    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    let shape_score = HashMap::from([
        ("X", 1), ("Y", 2), ("Z", 3)
    ]);

    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    // If both players choose the same shape, the round instead ends in a draw.
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    let round_score = HashMap::from([
        ("AX", 3), ("AY", 6), ("AZ", 0),
        ("BX", 0), ("BY", 3), ("BZ", 6),
        ("CX", 6), ("CY", 0), ("CZ", 3)
    ]);

    let mut splitted_lines_mutable: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        assert_eq!(splitted.len(), 2);

        splitted_lines_mutable.push(splitted);
    }

    let splitted_lines = splitted_lines_mutable.as_slice();

    let mut total_score: u64 = 0;
    for line in splitted_lines {
        let round = format!("{}{}", line[0], line[1]);

        total_score += shape_score[line[1]];
        total_score += round_score[round.as_str()];
    }

    print!("Part 1 - total score: {}\n", total_score);

    // X means you need to lose, Y means you need to end the round in a draw,
    // and Z means you need to win
    let result_lookup = HashMap::from([
        ("AX", "Z"), ("AY", "X"), ("AZ", "Y"),
        ("BX", "X"), ("BY", "Y"), ("BZ", "Z"),
        ("CX", "Y"), ("CY", "Z"), ("CZ", "X")
    ]);

    total_score = 0;

    for line in splitted_lines {
        let shape_to_choose_input = format!("{}{}", line[0], line[1]);
        let shape_to_choose = result_lookup[shape_to_choose_input.as_str()];

        let round = format!("{}{}", line[0], shape_to_choose);

        total_score += shape_score[shape_to_choose];
        total_score += round_score[round.as_str()];
    }

    print!("Part 2 - total score: {}\n", total_score);
}

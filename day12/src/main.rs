use pathfinding::prelude::{bfs, Matrix};

fn main() {
    println!("Day 12");

    let input = include_str!("../input/part1.txt");

    let mut hill = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = hill.keys().find(|p| hill[*p] == b'S').unwrap();
    let end = hill.keys().find(|p| hill[*p] == b'E').unwrap();
    hill[start] = b'a';
    hill[end] = b'z';
    let hill = &hill;

    let steps_part1 = bfs(
        &start,
        |&current_point| {
            hill.neighbours(current_point, false).filter(move |&candidate| hill[candidate] <= hill[current_point] + 1)
        },
        |&p| p == end,
    )
    .unwrap()
    .len() - 1;

    println!("part 1: {}", steps_part1);

    let steps_part2 = bfs(
        &end,
        |&current_point| {
            hill.neighbours(current_point, false).filter(move |&candidate| hill[current_point] <= hill[candidate] + 1)
        },
        |&p| hill[p] == b'a',
    )
    .unwrap()
    .len() - 1;

    println!("part 2: {}", steps_part2);
}

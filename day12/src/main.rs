use pathfinding::prelude::{astar, Matrix};
fn main() {
    println!("Day 12");

    let input = include_str!("../example/part1.txt");
    let hill = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = hill.keys().find(|p| hill[*p] == b'S').unwrap();
    let end = hill.keys().find(|p| hill[*p] == b'E').unwrap();

    println!("{:?}", hill);
    println!("start: {:?}", start);
    println!("end: {:?}", end);
}

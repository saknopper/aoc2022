use sscanf::sscanf;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    beacon_x: isize,
    beacon_y: isize,
    safe_distance: isize, // represented as Manhattan distance
}
fn main() {
    println!("Day 15");

    let input = include_str!("../example/part1.txt");

    let mut beacons = parse_input(input);

    beacons.iter().for_each(|b| println!("{:?}", b));
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|l| {
            let (s_x, s_y, b_x, b_y) =
                sscanf!(l, "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}").unwrap();
            let distance = (s_x - b_x).abs() + (s_y - b_y).abs();
            Sensor { x: s_x, y: s_y, beacon_x: b_x, beacon_y: b_y, safe_distance: distance }
        })
        .collect()
}

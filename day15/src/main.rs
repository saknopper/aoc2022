use std::collections::HashSet;

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

    let input = include_str!("../input/part1.txt");

    let sensors = parse_input(input);

    let mut positions_without_beacon: HashSet<(isize, isize)> = HashSet::new();
    sensors.iter().for_each(|s| sensor_get_positions_without_beacon(s, &mut positions_without_beacon, 2000000));
    // remove all positions where an actual beacon is present
    sensors.iter().for_each(|s| {
        positions_without_beacon.remove(&(s.beacon_y, s.beacon_x));
    });
    let part1 = positions_without_beacon.len();
    println!("{:?}", part1);
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

fn sensor_get_positions_without_beacon(
    sensor: &Sensor,
    positions: &mut HashSet<(isize, isize)>,
    row_of_interest: isize,
) {
    for row in (sensor.y - sensor.safe_distance)..=(sensor.y + sensor.safe_distance) {
        if row != row_of_interest {
            continue;
        }

        let column_width = ((sensor.y - row).abs() - sensor.safe_distance).abs();
        //println!("row {}, column width {}", row, column_width);
        for column in (sensor.x - column_width)..=(sensor.x + column_width) {
            //println!("checking {},{}", row, column);
            positions.insert((row, column));
        }
    }
}

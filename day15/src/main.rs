use std::{collections::HashMap, ops::RangeInclusive};

use sscanf::sscanf;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    beacon_x: isize,
    beacon_y: isize,
    safe_distance: isize, // represented as Manhattan distance
}

const ROW_OF_INTEREST: isize = 2000000;
const COORDS_MAX: isize = 4000000;

fn main() {
    println!("Day 15");

    let input = include_str!("../input/part1.txt");

    let sensors = parse_input(input);

    let mut row_ranges: HashMap<isize, Vec<RangeInclusive<isize>>> = HashMap::new();
    sensors.iter().for_each(|s| {
        sensor_get_range_without_beacon(s, &mut row_ranges, ROW_OF_INTEREST, ROW_OF_INTEREST, isize::MIN, isize::MAX)
    });
    let row = row_ranges.get_mut(&ROW_OF_INTEREST).unwrap();
    let min_x = row.iter().map(|r| r.start()).min().unwrap();
    let max_x = row.iter().map(|r| r.end()).max().unwrap();

    let part1 = max_x - min_x;
    println!("part 1: {:?}", part1);

    let mut row_ranges: HashMap<isize, Vec<RangeInclusive<isize>>> = HashMap::new();
    let mut beacon_pos: (isize, isize) = (0, 0);
    sensors.iter().for_each(|s| sensor_get_range_without_beacon(s, &mut row_ranges, 0, COORDS_MAX, 0, COORDS_MAX));
    let mut row_ranges_iter = row_ranges.iter_mut();
    while let Some((row, ranges)) = row_ranges_iter.next() {
        ranges.sort_by(|a, b| a.start().cmp(b.start()));
        let mut max_so_far: isize = 0;
        let mut iter = ranges.windows(2);
        while let Some(pair) = iter.next() {
            if pair[1].start() - pair[0].end().max(&max_so_far) == 2 {
                beacon_pos = (*row, pair[0].end() + 1);
            }
            max_so_far = *pair[0].end().max(&max_so_far);
        }
    }

    println!("part 2: {:?}", (beacon_pos.1 as i64 * 4000000 as i64) + beacon_pos.0 as i64);
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

fn sensor_get_range_without_beacon(
    sensor: &Sensor,
    row_ranges: &mut HashMap<isize, Vec<RangeInclusive<isize>>>,
    min_row: isize,
    max_row: isize,
    min_column: isize,
    max_column: isize,
) {
    for row in (sensor.y - sensor.safe_distance).max(min_row)..=(sensor.y + sensor.safe_distance).min(max_row) {
        let column_width = ((sensor.y - row).abs() - sensor.safe_distance).abs();
        let column_range = (sensor.x - column_width).max(min_column)..=(sensor.x + column_width).min(max_column);
        let row = row_ranges.entry(row).or_insert(Vec::new());
        row.push(column_range);
    }

    let row = row_ranges.entry(sensor.beacon_y).or_insert(Vec::new());
    row.push(sensor.beacon_x..=sensor.beacon_x);
}

use std::{collections::BTreeMap, ops::RangeInclusive};

#[derive(Debug, PartialEq)]
enum Tile {
    AIR,
    ROCK,
    SAND,
    VOID,
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    println!("Day 14");

    let input = include_str!("../input/part1.txt");
    let (mut cave, _) = parse_cave(input);

    let mut unit_counter: usize = 0;
    while do_sand_simulation(&mut cave, 0) {
        unit_counter += 1;
    }

    println!("part 1: {}", unit_counter);

    let (mut cave, max_y) = parse_cave(input);
    cave.values_mut().for_each(|column| {
        column.insert(max_y + 2, Tile::ROCK);
    });
    let mut unit_counter: usize = 0;
    while do_sand_simulation(&mut cave, max_y + 2) {
        unit_counter += 1;
    }

    println!("part 2: {}", unit_counter + 1);
}

fn do_sand_simulation(cave: &mut BTreeMap<isize, BTreeMap<isize, Tile>>, floor: isize) -> bool {
    let mut current_pos = Point { x: 500, y: 0 };
    let positions_to_check: Vec<(isize, isize)> = vec![(0, 1), (-1, 1), (1, 1)];
    let positions_to_check = positions_to_check.as_slice();

    'outer: loop {
        for checking in positions_to_check {
            match *get_tile_at_position(
                cave,
                Point { x: current_pos.x + checking.0, y: current_pos.y + checking.1 },
                floor,
            ) {
                Tile::VOID => {
                    return false;
                }
                Tile::AIR => {
                    current_pos = Point { x: current_pos.x + checking.0, y: current_pos.y + checking.1 };
                    continue 'outer;
                }
                _ => {}
            }
        }

        cave.get_mut(&current_pos.x).unwrap().insert(current_pos.y, Tile::SAND);
        if current_pos.x == 500 && current_pos.y == 0 {
            return false;
        }

        break;
    }

    true
}

fn get_tile_at_position(cave: &mut BTreeMap<isize, BTreeMap<isize, Tile>>, p: Point, floor: isize) -> &Tile {
    if floor != 0 && cave.get(&p.x).is_none() {
        let mut new_column = BTreeMap::new();
        new_column.insert(floor, Tile::ROCK);
        cave.insert(p.x, new_column);
    }

    match cave.get(&p.x) {
        None => &Tile::VOID,
        Some(column) => match column.get(&p.y) {
            None => &Tile::AIR,
            Some(tile) => tile,
        },
    }
}

fn parse_cave(input: &str) -> (BTreeMap<isize, BTreeMap<isize, Tile>>, isize) {
    let mut paths: Vec<Vec<Point>> = Vec::new();

    input.lines().for_each(|line| {
        let points = line
            .split(" -> ")
            .map(|pstr| {
                let coords = pstr.split(',').collect::<Vec<_>>();
                Point {
                    x: coords.get(0).unwrap().parse::<isize>().unwrap(),
                    y: coords.get(1).unwrap().parse::<isize>().unwrap(),
                }
            })
            .collect::<Vec<_>>();
        paths.push(points);
    });
    let min_x = paths.iter().flat_map(|points| points.iter()).map(|p| p.x).min().unwrap();
    let max_x = paths.iter().flat_map(|points| points.iter()).map(|p| p.x).max().unwrap();
    let max_y = paths.iter().flat_map(|points| points.iter()).map(|p| p.y).max().unwrap();

    let mut cave: BTreeMap<isize, BTreeMap<isize, Tile>> = BTreeMap::new();
    for distance_right in min_x..=max_x {
        cave.insert(distance_right, BTreeMap::new());
    }

    paths.iter().for_each(|path| {
        let mut path_iter = path.windows(2);
        while let Some(points) = path_iter.next() {
            let lh = &points[0];
            let rh = &points[1];
            if lh.x == rh.x {
                // Vertical line
                let column = cave.get_mut(&lh.x).unwrap();
                for i in range_helper(lh.y, rh.y) {
                    column.insert(i, Tile::ROCK);
                }
            } else {
                // Horizontal line
                for i in range_helper(lh.x, rh.x) {
                    let column = cave.get_mut(&i).unwrap();
                    column.insert(lh.y, Tile::ROCK);
                }
            }
        }
    });

    (cave, max_y)
}

fn range_helper(l: isize, r: isize) -> RangeInclusive<isize> {
    if l < r {
        l..=r
    } else {
        r..=l
    }
}

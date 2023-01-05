use std::{collections::BTreeMap, ops::RangeInclusive};

#[derive(Debug)]
enum Tile {
    AIR,
    ROCK,
    SAND,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    println!("Day 14");

    let input = include_str!("../example/part1.txt");
    let mut cave = parse_cave(input);

    cave.iter().for_each(|col| println!("{:?}", col));

    // Do sand simulation
}

fn parse_cave(input: &str) -> BTreeMap<usize, BTreeMap<usize, Tile>> {
    let mut paths: Vec<Vec<Point>> = Vec::new();

    input.lines().for_each(|line| {
        let points = line
            .split(" -> ")
            .map(|pstr| {
                let coords = pstr.split(',').collect::<Vec<_>>();
                Point {
                    x: coords.get(0).unwrap().parse::<usize>().unwrap(),
                    y: coords.get(1).unwrap().parse::<usize>().unwrap(),
                }
            })
            .collect::<Vec<_>>();
        paths.push(points);
    });
    let min_x = paths.iter().flat_map(|points| points.iter()).map(|p| p.x).min().unwrap();
    let max_x = paths.iter().flat_map(|points| points.iter()).map(|p| p.x).max().unwrap();

    let mut cave: BTreeMap<usize, BTreeMap<usize, Tile>> = BTreeMap::new();
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

    cave
}

fn range_helper(l: usize, r: usize) -> RangeInclusive<usize> {
    if l < r {
        l..=r
    } else {
        r..=l
    }
}

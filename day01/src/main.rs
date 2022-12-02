use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::usize;

fn main() {
    println!("Day 1");

    let path = Path::new("input/part1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("read file {}\n", display),
    }

    let mut calories: Vec<u64> = Vec::new();
    calories.push(0);
    let mut index: usize = 0;
    for line in s.lines() {
        if line.is_empty() {
            index += 1;
            calories.push(0);
            continue;
        }

        calories[index] += line.parse::<u64>().unwrap();
    }

    calories.sort();

    print!("part 1: max calories: {}\n", calories.last().unwrap());

    let sum : u64 = calories.iter().rev().take(3).sum();
    print!("part 2: sum of 3 max calories: {}\n", sum);
}

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use substring::Substring;

fn main() {
    println!("Day 6");

    let path = Path::new("input/part1.txt");
    let mut file = File::open(&path).expect("Couldn't open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't load file");

    print!("Part 1 - {}\n", find_marker_with_size(4, &s));
    print!("Part 2 - {}\n", find_marker_with_size(14, &s));
}

fn find_marker_with_size(marker_size: u32, input : &String) -> u32 {
    for i in 0..(input.len() - marker_size as usize) {
        let mut char_set = HashSet::new();

        let possible_marker = input.substring(i, i + marker_size as usize);
        possible_marker.chars().for_each(|c| { char_set.insert(c); });
        if char_set.len() == marker_size as usize {
            return i as u32 + marker_size;
        }
    }

    return 0;
}
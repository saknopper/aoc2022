use std::collections::HashSet;
use substring::Substring;

fn main() {
    println!("Day 6");

    let input = include_str!("../input/part1.txt");

    print!("Part 1 - {}\n", find_marker_with_size(4, input));
    print!("Part 2 - {}\n", find_marker_with_size(14, input));
}

fn find_marker_with_size(marker_size: u32, input : &str) -> u32 {
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
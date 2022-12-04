use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Day 3");

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

    let possible_items_tmp = ('a' as u8..'z' as u8 + 1)
                                        .chain('A' as u8..'Z' as u8 + 1)
                                        .map(|i| i as char)
                                        .collect::<Vec<_>>();
    let possible_items = possible_items_tmp.as_slice();

    let mut total_score : u64 = 0;
    for line in s.lines() {
        let compartments = line.split_at(line.len() / 2);

        for (i, item) in possible_items.iter().enumerate() {
            if compartments.0.contains(*item) && compartments.1.contains(*item) {
                total_score += i as u64 + 1;
                break;
            }
        }
    }

    print!("Part 1 - score is: {}\n", total_score);

    total_score = 0;
    let lines : Vec<&str> = s.lines().into_iter().collect();
    for group in lines.chunks(3) {
        for (i, item) in possible_items.iter().enumerate() {
            if group.iter().all(|r| r.contains(*item)) {
                total_score += i as u64 + 1;
                break;
            }
        }
    }

    print!("Part 1 - score is: {}\n", total_score);
}

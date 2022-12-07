use std::collections::HashMap;

fn main() {
    println!("Day 7");

    let input = include_str!("../input/part1.txt");
    let paths_and_sizes = parse_input(input);

    print!(
        "Part 1 - {}\n",
        paths_and_sizes
            .values()
            .filter(|val| **val <= 100000)
            .sum::<usize>()
    );

    let used_space = paths_and_sizes.get("").unwrap();
    let minimal_cleanup_required = used_space - (70000000 - 30000000);

    let mut sorted_sizes = paths_and_sizes.values().copied().collect::<Vec<usize>>();
    sorted_sizes.sort();

    print!(
        "Part 2 - {}\n",
        sorted_sizes
            .iter()
            .find(|val| **val >= minimal_cleanup_required)
            .unwrap()
    );
}

fn parse_input(input: &str) -> HashMap<String, usize> {
    let mut paths_and_sizes: HashMap<String, usize> = HashMap::new();

    let mut current_path = Vec::<&str>::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let cmd = line.split_at(2).1;
            let (action, dst) = cmd.split_once(' ').unwrap_or_default();
            if action == "cd" {
                if dst == "/" {
                    current_path.clear();
                } else if dst == ".." {
                    current_path.pop().unwrap();
                } else {
                    current_path.push(dst);
                }
            }
        } else {
            if !line.starts_with("dir ") {
                let (size, _filename) = line.split_once(' ').unwrap();
                let size = size.parse::<usize>().unwrap();

                for p in 0..=current_path.len() {
                    let path = current_path[..p].join("/").to_string();

                    paths_and_sizes
                        .entry(path)
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
            }
        }
    }

    paths_and_sizes
}

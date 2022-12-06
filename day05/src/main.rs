use scanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("Day 5");

    let path = Path::new("input/part1.txt");
    let mut file = File::open(&path).expect("Couldn't open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't load file");

    print!("Part 1: {}\n", move_and_get_top_containers(&s, false));
    print!("Part 2: {}\n", move_and_get_top_containers(&s, true));
}

fn move_and_get_top_containers(input: &String, multiple_at_once: bool) -> String {
    let splitted = input.split("\n\n").collect::<Vec<_>>();
    let container_input = splitted[0];
    let instructions_input = splitted[1];

    let amount_of_stacks = container_input
        .lines()
        .last()
        .expect("line not found")
        .chars()
        .last()
        .expect("char not found")
        .to_digit(10)
        .expect("error while parsing to digit");

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for i in 0..amount_of_stacks {
        let mut current_stack: Vec<char> = Vec::new();

        let line_index = if i == 0 { 1 } else { (i * 4) + 1 };
        let line_index = line_index as usize;
        for line in container_input.lines().rev().skip(1) {
            let _crate = line.chars().nth(line_index).unwrap_or(' ');
            if _crate != ' ' {
                current_stack.push(_crate);
            }
        }

        stacks.push(current_stack);
    }

    for instruction in instructions_input.lines() {
        let mut amount: u32 = 0;
        let mut src: usize = 0;
        let mut dst: usize = 0;
        sscanf!(instruction, "move {} from {} to {}", amount, src, dst)
            .expect("error parsing instruction");

        if multiple_at_once {
            let src_stack_len = stacks.get(src - 1).unwrap().len();
            let crates_to_move: Vec<char> = stacks
                .get_mut(src - 1)
                .expect("can't find src stack")
                .drain((src_stack_len - amount as usize)..src_stack_len)
                .collect();
            let dst_stack = stacks.get_mut(dst - 1).expect("can't find dst stack");
            crates_to_move.into_iter().for_each(|c| dst_stack.push(c));
        } else {
            for _ in 0..amount {
                let crate_to_move = stacks
                    .get_mut(src - 1)
                    .expect("can't find src stack")
                    .pop()
                    .expect("no crate left");

                stacks
                    .get_mut(dst - 1)
                    .expect("can't find dst stack")
                    .push(crate_to_move);
            }
        }
    }

    let mut result: String = String::new();
    for s in &stacks {
        result.push(*(s.last().expect("no element")));
    }

    return result;
}

use std::{cell::RefCell, collections::VecDeque};

#[derive(Debug)]
enum Operation {
    ADD,
    MULTIPLY,
    SQUARED,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    operation_rh: usize,
    test_divisor: usize,
    test_fail_monkey: usize,
    test_success_monkey: usize,
    inspection_count: usize,
}
fn main() {
    println!("Day 11");

    let input = include_str!("../input/part1.txt");
    let splitted_by_monkey = input.split("\n\n").collect::<Vec<_>>();

    let mut monkeys: Vec<RefCell<Monkey>> = Vec::new();
    parse_input(&splitted_by_monkey, &mut monkeys);

    for _ in 0..20 {
        let mut monkeys_iter = monkeys.iter();
        while let Some(monkey_cell) = monkeys_iter.next() {
            let mut monkey = monkey_cell.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                let new_item_worry = match monkey.operation {
                    Operation::ADD => item + monkey.operation_rh,
                    Operation::MULTIPLY => item * monkey.operation_rh,
                    Operation::SQUARED => item * item,
                };

                let new_item_worry = new_item_worry / 3;

                let dst_monkey = match new_item_worry % monkey.test_divisor {
                    0 => monkey.test_success_monkey,
                    _ => monkey.test_fail_monkey,
                };

                monkeys
                    .iter()
                    .nth(dst_monkey)
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push_back(new_item_worry);

                monkey.inspection_count += 1;
            }
        }
    }

    monkeys.iter().for_each(|m| println!("{:?}", m));

    monkeys.sort_by(|a, b| {
        a.borrow()
            .inspection_count
            .cmp(&b.borrow().inspection_count)
    });

    let monkey_business: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.borrow().inspection_count)
        .reduce(|a, b| a * b)
        .unwrap();
    println!("part 1: {monkey_business}");
}

fn parse_input(splitted_by_monkey: &Vec<&str>, monkeys: &mut Vec<RefCell<Monkey>>) {
    for monkey_input in splitted_by_monkey {
        let mut monkey_lines = monkey_input.lines();
        monkey_lines.next();
        let starting_items = monkey_lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .into_iter()
            .map(|val| val.trim())
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();
        let (operation, operation_rh) = monkey_lines
            .next()
            .unwrap()
            .split_once("new = ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
            .1
            .split_once(' ')
            .unwrap();
        let operation = match operation {
            "+" => Operation::ADD,
            "*" => Operation::MULTIPLY,
            _ => unreachable!(),
        };

        let test = monkey_lines
            .next()
            .unwrap()
            .split_once("divisible by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let test_success = monkey_lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();
        let test_fail = monkey_lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();

        monkeys.push(RefCell::new(Monkey {
            items: starting_items,
            operation: if operation_rh == "old" {
                Operation::SQUARED
            } else {
                operation
            },
            operation_rh: if operation_rh == "old" {
                0
            } else {
                operation_rh.parse::<usize>().unwrap()
            },
            test_divisor: test,
            test_fail_monkey: test_fail as usize,
            test_success_monkey: test_success as usize,
            inspection_count: 0,
        }));
    }
}

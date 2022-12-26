use std::collections::VecDeque;
use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut input_buffer: Vec<String> = Vec::new();
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "" | "-" => {
                let starting_items_line = &input_buffer[0];
                let operation_line = &input_buffer[1];
                let test_line = &input_buffer[2];
                let target_true_line = &input_buffer[3];
                let target_false_line = &input_buffer[4];

                let (_, item_list_str) = starting_items_line.split_once(": " ).unwrap();
                let item_list: VecDeque<usize> = item_list_str.split(", ").map(|item| usize::from_str(item).unwrap()).collect();

                let (_, operation_str) = operation_line.split_once("new = old ").unwrap();
                let (operator, constant) = operation_str.split_once(" ").unwrap();
                let operation = match (operator, constant) {
                    ("*", "old") => {
                        Box::new(move |input| {input * input}) as Box<dyn Fn(usize) -> usize>
                    },
                    ("+", "old") => {
                        Box::new(move |input| {input + input}) as Box<dyn Fn(usize) -> usize>
                    },
                    ("*", x) => {
                        let x = usize::from_str(x).unwrap();
                        Box::new(move |input| {input * x}) as Box<dyn Fn(usize) -> usize>
                    },
                    ("+", x) => {
                        let x = usize::from_str(x).unwrap();
                        Box::new(move |input| {input + x}) as Box<dyn Fn(usize) -> usize>
                    },
                    _ => unreachable!("Unsupported operator(?)"),
                };

                let (_, test_str) = test_line.split_once("divisible by ").unwrap();
                let test_mod = usize::from_str(test_str).unwrap();

                let (_, true_target_str) = target_true_line.split_once(" throw to monkey ").unwrap();
                let true_target = usize::from_str(true_target_str).unwrap();

                let (_, false_target_str) = target_false_line.split_once(" throw to monkey ").unwrap();
                let false_target = usize::from_str(false_target_str).unwrap();

                monkeys.push(Monkey {
                    items: item_list,
                    inspected_count: 0,
                    operation,
                    test_mod,
                    target_test_true: true_target,
                    target_test_false: false_target,
                });

                input_buffer.clear();

                if line == "-" {
                    break;
                }
            }
            _ => {
                if line.starts_with("Monkey") {
                    input_buffer.clear();
                } else {
                    input_buffer.push(line);
                }
            }
        }
    }

    let mut common = lcm(monkeys[0].test_mod, monkeys[1].test_mod);
    for i in 1..monkeys.len() {
        common = lcm(common, monkeys[i].test_mod);
    }

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspected_count += 1;
                let worry = (*monkeys[i].operation)(item) % common;
                match worry % monkeys[i].test_mod == 0 {
                    true => {
                        let target = monkeys[i].target_test_true;
                        monkeys[target].items.push_back(worry);
                    },
                    false => {
                        let target = monkeys[i].target_test_false;
                        monkeys[target].items.push_back(worry);
                    }
                }
            }
        }
    }

    let mut inspected = monkeys.iter().map(|monkey| monkey.inspected_count).collect::<Vec<usize>>();
    inspected.sort();
    println!("{}", inspected.pop().unwrap() * inspected.pop().unwrap());
}

pub struct Monkey {
    items: VecDeque<usize>,
    inspected_count: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test_mod: usize,
    target_test_true: usize,
    target_test_false: usize,
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
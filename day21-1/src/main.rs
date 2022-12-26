use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut monkeys = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let (name, operation) = line.split_once(": ").unwrap();
                match usize::from_str(operation) {
                    Ok(n) => {
                        monkeys.insert(name.to_string(), Monkey::Number(n));
                    }
                    Err(_err) => {
                        let op = operation.chars().skip(5).next().unwrap();
                        monkeys.insert(name.to_string(), Monkey::Math((operation[0..4].to_string(), op, operation[7..].to_string())));
                    }
                };
            }
        }
    }

    println!("{}", resolve("root", &monkeys));
}

pub fn resolve(name: &str, monkeys: &HashMap<String, Monkey>) -> usize {
    match monkeys.get(name).unwrap() {
        Monkey::Math((name1, op, name2)) => {
            let n1 = resolve(&name1, monkeys);
            let n2 = resolve(&name2, monkeys);
            match op {
                '+' => n1 + n2,
                '-' => n1 - n2,
                '*' => n1 * n2,
                '/' => n1 / n2,
                _ => unreachable!()
            }
        }
        Monkey::Number(n) => *n,
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Monkey {
    Math((String, char, String)),
    Number(usize),
}
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

    let root = match monkeys.get("root").unwrap() {
        Monkey::Math((name1, _op, name2)) => (name1, name2),
        Monkey::Number(_) => panic!(),
    };
    let n1 = resolve(root.0, &monkeys);
    let n2 =  resolve(root.1, &monkeys);
    match n1 {
        Some(n1) => {
            println!("{:?}", resolve_rev(root.1, &monkeys, n1));
        }
        None => { }
    }
    match n2 {
        Some(n2) => {
            println!("{:?}", resolve_rev(root.0, &monkeys, n2));
        }
        None => {}
    }
}

pub fn resolve(name: &str, monkeys: &HashMap<String, Monkey>) -> Option<usize> {
    if name == "humn" {
        return None;
    }
    match monkeys.get(name).unwrap() {
        Monkey::Math((name1, op, name2)) => {
            let n1 = resolve(&name1, monkeys)?;
            let n2 = resolve(&name2, monkeys)?;
            Some(match op {
                '+' => n1 + n2,
                '-' => n1 - n2,
                '*' => n1 * n2,
                '/' => n1 / n2,
                _ => unreachable!()
            })
        }
        Monkey::Number(n) => Some(*n),
    }
}

pub fn resolve_rev(name: &str, monkeys: &HashMap<String, Monkey>, number: usize) -> usize {
    if name == "humn" {
        return number;
    }
    match monkeys.get(name).unwrap() {
        Monkey::Math((name1, op, name2)) => {
            let n1 = resolve(&name1, monkeys);
            let n2 = resolve(&name2, monkeys);
            match op {
                '+' => {
                    match n1 {
                        Some(n1) => {
                            resolve_rev(&name2, monkeys, number - n1)
                        }
                        None => {
                            resolve_rev(&name1, monkeys, number - n2.unwrap())
                        }
                    }
                }
                '-' => {
                    match n1 {
                        Some(n1) => {
                            resolve_rev(&name2, monkeys, n1 - number)
                        }
                        None => {
                            resolve_rev(&name1, monkeys, number + n2.unwrap())
                        }
                    }
                }
                '*' => {
                    match n1 {
                        Some(n1) => {
                            resolve_rev(&name2, monkeys, number / n1)
                        }
                        None => {
                            resolve_rev(&name1, monkeys, number / n2.unwrap())
                        }
                    }
                }
                '/' => {
                    match n1 {
                        Some(n1) => {
                            resolve_rev(&name2, monkeys, n1 / number)
                        }
                        None => {
                            resolve_rev(&name1, monkeys, number * n2.unwrap())
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        Monkey::Number(_n) => {
            unreachable!()
        },
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Monkey {
    Math((String, char, String)),
    Number(usize),
}
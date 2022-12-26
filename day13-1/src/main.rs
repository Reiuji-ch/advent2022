use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut score = 0;
    let mut idx = 0;
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            },
            "" => {},
            line => {
                idx += 1;
                let list_left_str = line;
                let list_right_str = lines.next().unwrap().unwrap();
                let list_left = parse_list(&list_left_str);
                let list_right = parse_list(&list_right_str);

                match compare_lists(&list_left, &list_right) {
                    CompareResult::RightOrder(_) => {
                        score += idx;
                    }
                    CompareResult::WrongOrder => {}
                    CompareResult::Continue => {
                        panic!("Failed to determine correctness for {idx}");
                    }
                }
            }
        }
    }
    println!("{score}");
}

#[derive(Debug)]
enum ListElement {
    List(Vec<ListElement>),
    Atom(usize),
}

fn parse_list(list: &str) -> Vec<ListElement> {
    let trimmed_list = &list[1..list.len()-1];
    let mut level = 0;
    let mut output = Vec::new();

    let mut idx = 0;
    let mut start_idx = usize::MAX;
    for ch in trimmed_list.chars() {
        match ch {
            '[' => {
                level += 1;
                if level == 1 {
                    start_idx = idx;
                }
            },
            ']' => {
                level -= 1;
                if level == 0 {
                    output.push(ListElement::List(parse_list(&trimmed_list[start_idx..idx+1])));
                    start_idx = usize::MAX;
                }
            }
            ',' => {
                if level != 0 || start_idx == usize::MAX {
                    idx += 1;
                    continue;
                }
                output.push(ListElement::Atom(usize::from_str(&trimmed_list[start_idx..idx]).unwrap()));
                start_idx = usize::MAX;
            }
            _ => {
                if level != 0 {
                    idx += 1;
                    continue;
                }
                if start_idx == usize::MAX {
                    start_idx = idx;
                }
            }
        }
        idx += 1;
    }
    if level != 0 {
        panic!("Parse error");
    }

    // Remember the last element
    if start_idx != usize::MAX {
        output.push(ListElement::Atom(usize::from_str(&trimmed_list[start_idx..idx]).unwrap()));
    }

    output
}

#[derive(Debug)]
enum CompareResult {
    RightOrder(usize),
    WrongOrder,
    Continue,
}

fn compare_lists(left: &Vec<ListElement>, right: &Vec<ListElement>) -> CompareResult {
    let mut index = 0;
    let mut iter_left = left.iter();
    let mut iter_right = right.iter();
    loop {
        index += 1;
        match (iter_left.next(), iter_right.next()) {
            (None, None) => {
                return CompareResult::Continue;
            }
            (None, Some(_)) => {
                return CompareResult::RightOrder(index);
            },
            (Some(_), None) => {
                return CompareResult::WrongOrder;
            },
            (Some(ListElement::Atom(n_left)), Some(ListElement::Atom(n_right))) => {
                if n_left < n_right {
                    return CompareResult::RightOrder(index);
                } else if n_right < n_left {
                    return CompareResult::WrongOrder;
                }
            },
            (Some(ListElement::Atom(n)), Some(ListElement::List(list))) => {
                match compare_lists(&vec![ListElement::Atom(*n)], &list) {
                    CompareResult::WrongOrder => {
                        return CompareResult::WrongOrder;
                    }
                    CompareResult::Continue => {},
                    CompareResult::RightOrder(idx) => {
                        return CompareResult::RightOrder(index+idx-1);
                    }
                }
            },
            (Some(ListElement::List(list)), Some(ListElement::Atom(n))) => {
                match compare_lists(&list, &vec![ListElement::Atom(*n)]) {
                    CompareResult::WrongOrder => {
                        return CompareResult::WrongOrder;
                    }
                    CompareResult::Continue => {},
                    CompareResult::RightOrder(idx) => {
                        return CompareResult::RightOrder(index+idx-1);
                    }
                }
            },
            (Some(ListElement::List(list_l)), Some(ListElement::List(list_r))) => {
                match compare_lists(&list_l, &list_r) {
                    CompareResult::WrongOrder => {
                        return CompareResult::WrongOrder;
                    }
                    CompareResult::Continue => {},
                    CompareResult::RightOrder(idx) => {
                        return CompareResult::RightOrder(index+idx-1);
                    }
                }
            },
        }
    }
}
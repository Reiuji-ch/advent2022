use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut visited = vec![(0,0)];
    let (mut head_x, mut head_y, mut tail_x, mut tail_y) = (0isize, 0isize, 0isize ,0isize);
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let (direction, moves) = line.split_once(" ").expect("No delimiter");
                let move_count = usize::from_str(moves).expect("Invalid number");

                for _ in 0..move_count {
                    match direction {
                        "R" => {
                            head_x += 1;
                        },
                        "L" => {
                            head_x -= 1;
                        },
                        "D" => {
                            head_y -= 1;
                        },
                        "U" => {
                            head_y += 1;
                        },
                        _ => unreachable!(),
                    }
                    let adjacent_x = (head_x - tail_x).abs() <= 1 && (head_y == tail_y);
                    let adjacent_y = (head_y - tail_y).abs() <= 1 && (head_x == tail_x);
                    // If still adjacent after moving, we don't move the tail, so skip these checks
                    if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
                        continue;
                    }
                    match (adjacent_x, adjacent_y) {
                        (true, true) => {
                            unreachable!();
                        },
                        (true, false) => {
                            tail_y += (head_y - tail_y).signum();
                        },
                        (false, true) => {
                            tail_x += (head_x - tail_x).signum();
                        },
                        (false, false) => {
                            tail_y += (head_y - tail_y).signum();
                            tail_x += (head_x - tail_x).signum();
                        },
                    }
                    visited.push((tail_x, tail_y));
                }
            }
        }
    }

    // De-duplicate visited
    visited.sort();
    visited.dedup();

    println!("{}", visited.len());
}

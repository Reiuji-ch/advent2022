use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut visited = vec![(0,0)];
    let mut segments = vec![(0isize, 0isize); 10];
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
                            segments[0].0 += 1;
                        },
                        "L" => {
                            segments[0].0 -= 1;
                        },
                        "D" => {
                            segments[0].1 -= 1;
                        },
                        "U" => {
                            segments[0].1 += 1;
                        },
                        _ => unreachable!(),
                    }
                    for i in 1..segments.len() {
                        let adjacent_x = (segments[i-1].0 - segments[i].0).abs() <= 1 && (segments[i-1].1 == segments[i].1);
                        let adjacent_y = (segments[i-1].1 - segments[i].1).abs() <= 1 && (segments[i-1].0 == segments[i].0);
                        // If still adjacent after moving, we don't move the tail, so skip these checks
                        if (segments[i-1].0 - segments[i].0).abs() <= 1 && (segments[i-1].1 - segments[i].1).abs() <= 1 {
                            continue;
                        }
                        match (adjacent_x, adjacent_y) {
                            (true, true) => {
                                unreachable!();
                            },
                            (true, false) => {
                                segments[i].1 += (segments[i-1].1 - segments[i].1).signum();
                            },
                            (false, true) => {
                                segments[i].0 += (segments[i-1].0 - segments[i].0).signum();
                            },
                            (false, false) => {
                                segments[i].1 += (segments[i-1].1 - segments[i].1).signum();
                                segments[i].0 += (segments[i-1].0 - segments[i].0).signum();
                            },
                        }
                    }

                    visited.push(segments.last().unwrap().clone());
                }
            }
        }
    }

    // De-duplicate visited
    visited.sort();
    visited.dedup();

    println!("{}", visited.len());
}

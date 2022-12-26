use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut max = [0usize; 3];
    let mut acc = 0;
    for line in input.lines() {
        let line = line.expect("Read error");

        if line == "" || line == "-" {
            // Entirely too much functional programming to get the index and value of the lowest element
            let (min_idx, min_value) = max.iter().enumerate().fold((0, usize::MAX), |carry, value| {
                if value.1 < &carry.1 {
                    (value.0, *value.1)
                } else {
                    carry
                }
            });
            if acc > min_value {
                max[min_idx] = acc;
            }
            acc = 0;

            if line == "-" {
                break;
            }
        } else {
            acc += usize::from_str(&line).expect("Invalid number {line}");
        }
    }

    println!("{}", max.into_iter().sum::<usize>());
}
use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut signal = 1;
    let mut cycle = 1;
    let mut sum = 0;
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            },
            "noop" => {
                if cycle == 20 || (cycle+20) % 40 == 0 {
                    sum += signal * cycle;
                }
                cycle += 1;
            }
            _ => {
                let (_, number) = line.split_once(" ").expect("Invalid addx");
                let number = isize::from_str(number).unwrap();
                if cycle == 20 || (cycle+20) % 40 == 0 {
                    sum += signal * cycle;
                }
                cycle += 1;
                if cycle == 20 || (cycle+20) % 40 == 0 {
                    sum += signal * cycle;
                }
                cycle += 1;
                signal += number;
            }
        }
    }

    println!("{sum}");
}

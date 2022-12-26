use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut signal = 1isize;
    let mut cycle = 1;
    let mut display = [['.'; 40]; 6];
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            },
            "noop" => {
                let x_pos = (cycle-1) % 40;
                if (signal-x_pos).abs() <= 1 {
                    display[((cycle-1) / 40) as usize][x_pos as usize] = '#';
                }
                cycle += 1;
            }
            _ => {
                let (_, number) = line.split_once(" ").expect("Invalid addx");
                let number = isize::from_str(number).unwrap();
                let x_pos = (cycle-1) % 40;
                if (signal-x_pos).abs() <= 1 {
                    display[((cycle-1) / 40) as usize][x_pos as usize] = '#';
                }
                cycle += 1;
                let x_pos = (cycle-1) % 40;
                if (signal-x_pos).abs() <= 1 {
                    display[((cycle-1) / 40) as usize][x_pos as usize] = '#';
                }
                cycle += 1;
                signal += number;
            }
        }
    }

    for row in display {
        for x in row {
            print!("{x}");
        }
        println!();
    }
}

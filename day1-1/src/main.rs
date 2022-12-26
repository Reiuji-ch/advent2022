use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret an END OF TRANSMISSION character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut max = 0;
    let mut acc = 0;
    for line in input.lines() {
        let line = line.expect("Read error");
        if line == "" || line == "-" {
            max = max.max(acc);
            acc = 0;
            if line == "-" {
                break;
            }
        } else {
            acc += u64::from_str(&line).expect("Invalid number {line}");
        }
    }

    println!("{max}");
}
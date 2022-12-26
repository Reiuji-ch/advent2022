use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut sum = 0usize;
    for line in input.lines() {
        let line = line.expect("Read error");
        if line == "-" {
            break;
        }

        let (range_left, range_right) = line.split_once(",").expect("Invalid line");
        let (left_lower, left_upper) = range_left.split_once("-").expect("Invalid range");
        let (right_lower, right_upper) = range_right.split_once("-").expect("Invalid range");

        let l1 = usize::from_str(left_lower).unwrap();
        let l2 = usize::from_str(left_upper).unwrap();
        let r1 = usize::from_str(right_lower).unwrap();
        let r2 = usize::from_str(right_upper).unwrap();

        if (l1 <= r1 && l2 >= r2) || (r1 <= l1 && r2 >= l2) {
            sum += 1;
        }
    }

    println!("{sum}");
}
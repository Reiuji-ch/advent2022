use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut score = 0;
    for line in input.lines() {
        let line = line.expect("Read error");
        if line == "-" {
            break;
        }

        let (move_opponent, move_you) = line.split_once(" ").expect("Unexpected data in line");
        score += match move_you {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!()
        };

        score += match (move_opponent, move_you) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => unreachable!()
        };
    }

    println!("{score}");
}
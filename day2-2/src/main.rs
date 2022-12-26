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

        let (move_opponent, outcome) = line.split_once(" ").expect("Unexpected data in line");
        let move_you= match (move_opponent, outcome) {
            ("B", "X") | ("A", "Y") | ("C", "Z") => "A",
            ("C", "X") | ("B", "Y") | ("A", "Z") => "B",
            ("A", "X") | ("C", "Y") | ("B", "Z") => "C",
            _ => unreachable!()
        };

        score += match move_you {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!()
        };

        score += match (move_opponent, move_you) {
            ("A", "C") | ("B", "A") | ("C", "B") => 0,
            ("A", "A") | ("B", "B") | ("C", "C") => 3,
            ("A", "B") | ("B", "C") | ("C", "A") => 6,
            _ => unreachable!()
        };
    }

    println!("{score}");
}
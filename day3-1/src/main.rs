use std::io::BufRead;

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

        assert_eq!(line.len() % 2, 0);
        let (first_half, second_half) = line.split_at(line.len()/2);
        let duplicate = first_half.chars().find_map(|ch| {
            match second_half.contains(ch) {
                true => Some(ch),
                false => None,
            }
        }).expect("No duplicate character");

        // Do some ASCII magic to convert letter to priority
        assert!(duplicate.is_ascii());
        assert!(duplicate.is_alphabetic());
        sum += match duplicate as u8 {
            ascii_code @ 65 ..= 90 => ascii_code - 38,
            ascii_code @ 97 ..= 122 => ascii_code - 96,
            _ => unreachable!("Unexpected ASCII character {:?}", duplicate),
        } as usize;
    }

    println!("{sum}");
}
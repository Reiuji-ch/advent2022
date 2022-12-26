use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut sum = 0usize;
    loop {
        let line1 = lines.next().expect("No line").expect("Read error");
        if line1 == "-" {
            break;
        }
        let line2 = lines.next().expect("No line").expect("Read error");
        let line3 = lines.next().expect("No line").expect("Read error");

        let duplicates = line1.chars().filter_map(|ch| {
            match line2.contains(ch) {
                true => Some(ch),
                false => None,
            }
        }).collect::<Vec<char>>();

        let duplicate = duplicates.iter().find_map(|ch| {
            match line3.contains(*ch) {
                true => Some(ch),
                false => None,
            }
        }).expect("No duplicate char across all 3 lines");

        // Do some ASCII magic to convert letter to priority
        assert!(duplicate.is_ascii());
        assert!(duplicate.is_alphabetic());
        sum += match *duplicate as u8 {
            ascii_code @ 65 ..= 90 => ascii_code - 38,
            ascii_code @ 97 ..= 122 => ascii_code - 96,
            _ => unreachable!("Unexpected ASCII character {:?}", duplicate),
        } as usize;
    }

    println!("{sum}");
}
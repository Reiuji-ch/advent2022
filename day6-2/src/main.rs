use std::io::BufRead;

pub fn main() {
    let input = std::io::stdin().lock();
    let line = input.lines().next().expect("No lines").expect("Read error");

    let chars: Vec<char> = line.chars().collect();
    // Note we want the index of the _last_ character (1-indexed)
    // This means it can, at the earliest, be after 4 characters
    let mut counter = 14;
    for chars in chars.windows(14) {
        if chars.iter().fold(true, |carry, ch| chars.iter().filter(|c| c == &ch).count() == 1 && carry) {
            break;
        }
        counter += 1;
    }

    println!("{counter}");
}
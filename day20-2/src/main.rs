use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut numbers = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                numbers.push((numbers.len(), isize::from_str(&line).unwrap() * 811589153));
            }
        }
    }

    for _ in 0..10 {
        for iter in 0..numbers.len() {
            let (idx, ordering) = numbers.iter().enumerate().find_map(|elem| if elem.1.0 == iter {
                Some((elem.0, elem.1.0))
            } else {
                None
            }
            ).unwrap();
            let shift_amount = numbers[idx].1;
            numbers.remove(idx);
            let modu = numbers.len();
            let mut new_idx = ((((idx as isize) + shift_amount) % modu as isize) + modu as isize) as usize % modu;
            if new_idx == 0 {
                new_idx = numbers.len();
            }
            numbers.insert(new_idx as usize, (ordering, shift_amount));
        }
    }

    let idx = numbers.iter().enumerate().find_map(|elem| if elem.1.1 == 0 {
        Some(elem.0)
    } else {
        None
    }
    ).unwrap();

    println!("{}", numbers[(idx+1000) % numbers.len()].1 + numbers[(idx+2000) % numbers.len()].1 + numbers[(idx+3000) % numbers.len()].1)
}

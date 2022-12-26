use std::fmt::{Display, Formatter};
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
                numbers.push(Number::from_str(&line).unwrap());
            }
        }
    }

    println!("{}", Number(numbers.iter().fold(0, |acc, elem| acc + elem.0)));
}

#[derive(Debug)]
struct Number(usize);

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;
        for place in 0..s.len() {
            sum += 5isize.pow(place as u32) * match &s[s.len() - place - 1..s.len() - place] {
                "2" => 2,
                "1" => 1,
                "0" => 0,
                "-" => -1,
                "=" => -2,
                _ => unreachable!(),
            };
        }
        Ok(Number(sum as usize))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        let mut count = self.0 as isize;
        let mut iterations = 1;
        loop {
            if (0..iterations).fold(0, |acc, elem| {
                acc + 2 * 5usize.pow(elem)
            }) as isize >= count {
                break;
            } else {
                iterations += 1;
            }
        }

        for i in (0..iterations).rev() {
            let modu = 5usize.pow(i) as isize;
            // Find highest number the remaining "digits" can provide
            let rem_power = (0..i).fold(0, |acc, elem| {
                acc + 2 * 5usize.pow(elem)
            }) as isize;

            if count > rem_power && rem_power != 0 && modu > count {
                let n = (modu - count - 1) / rem_power + 1;
                match n {
                    1 => buf.push('1'),
                    2 => buf.push('2'),
                    _ => unreachable!(),
                }
                count -= n * modu;
            } else if count > rem_power {
                let mut n = count / modu;
                if (count - n * modu) > rem_power {
                    n += 1;
                }

                match n {
                    1 => buf.push('1'),
                    2 => buf.push('2'),
                    _ => unreachable!(),
                }
                count -= n * modu;
            } else if count < 0 {
                if count + modu <= rem_power {
                    if count + modu * 2 <= rem_power {
                        buf.push('=');
                        count += modu * 2;
                    } else {
                        buf.push('-');
                        count += modu;
                    }
                } else {
                    buf.push('0');
                }
            } else {
                match count / modu {
                    -2 => buf.push('='),
                    -1 => buf.push('-'),
                    0 => buf.push('0'),
                    1 => buf.push('1'),
                    2 => buf.push('2'),
                    _ => unreachable!()
                }
            }
        }
        f.write_str(&buf.split_whitespace().rev().collect::<String>())
    }
}
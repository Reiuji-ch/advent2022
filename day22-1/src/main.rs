use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut map_buf = Vec::new();
    let mut instructions_buf = String::new();
    let mut reading_map = true;
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            "" => {
                reading_map = false;
            }
            _ => {
                match reading_map {
                    true => {
                        map_buf.push(line);
                    }
                    false => {
                        instructions_buf = line;
                    }
                };
            }
        }
    }
    let max_width = map_buf.iter().fold(0, |acc, elem| { acc.max(elem.len()) });
    let mut map = Vec::new();
    for line in &map_buf {
        let count = line.len();
        let mut buf = Vec::new();
        for ch in line.chars() {
            buf.push(ch);
        }
        for _i in count..max_width {
            buf.push(' ');
        }
        map.push(buf);
    }

    let mut instructions = Vec::new();
    for part in instructions_buf.split_inclusive(&['L', 'R']) {
        let dir = match &part[part.len() - 1..] {
            "L" => Some(-1),
            "R" => Some(1),
            _ => None,
        };
        let amount = match dir.is_some() {
            true => {
                isize::from_str(&part[..part.len() - 1]).unwrap()
            }
            false => {
                isize::from_str(&part).unwrap()
            }
        };

        instructions.push(amount);
        if let Some(dir) = dir {
            instructions.push(dir);
        }
    }

    let mut rotate = false;
    let mut pos_x = map[0].iter().enumerate().find_map(|(idx, ch)| if *ch == '.' {
        Some(idx)
    } else {
        None
    }).unwrap();
    let mut pos_y = 0;
    let mut facing = 0;
    for ins in instructions {
        match rotate {
            true => {
                facing = (facing + ins).rem_euclid(4);
            }
            false => {
                match facing {
                    // Right
                    0 => {
                        for _ in 0..ins {
                            if pos_x < max_width - 1 {
                                let ch = &map[pos_y][pos_x + 1];
                                if *ch == '.' {
                                    pos_x += 1;
                                    continue;
                                } else if *ch == '#' {
                                    break;
                                }
                            }
                            // Wrap
                            let wrap_to = map[pos_y][..pos_x].iter().enumerate().rev().find_map(|(idx, ch)| {
                                match ch {
                                    ' ' => Some(idx + 1),
                                    _ => None,
                                }
                            }).unwrap_or(0);
                            if map[pos_y][wrap_to] != '#' {
                                pos_x = wrap_to;
                            } else {
                                break;
                            }
                        }
                    }
                    // Down
                    1 => {
                        for _ in 0..ins {
                            if pos_y < map.len() - 1 {
                                let ch = &map[pos_y + 1][pos_x];
                                if *ch == '.' {
                                    pos_y += 1;
                                    continue;
                                } else if *ch == '#' {
                                    break;
                                }
                            }
                            // Wrap
                            let wrap_to = map[..pos_y].iter().enumerate().rev().find_map(|(idx, ch)| {
                                match ch[pos_x] {
                                    ' ' => Some(idx + 1),
                                    _ => None,
                                }
                            }).unwrap_or(0);
                            if map[wrap_to][pos_x] != '#' {
                                pos_y = wrap_to;
                            } else {
                                break;
                            }
                        }
                    }
                    // Left
                    2 => {
                        for _ in 0..ins {
                            if pos_x >= 1 {
                                let ch = &map[pos_y][pos_x - 1];
                                if *ch == '.' {
                                    pos_x -= 1;
                                    continue;
                                } else if *ch == '#' {
                                    break;
                                }
                            }
                            // Wrap
                            let wrap_to = map[pos_y][pos_x..].iter().enumerate().find_map(|(idx, ch)| {
                                match ch {
                                    ' ' => {
                                        Some(idx + pos_x - 1)
                                    },
                                    _ => None,
                                }
                            }).unwrap_or(max_width-1);
                            if map[pos_y][wrap_to] != '#' {
                                pos_x = wrap_to;
                            } else {
                                break;
                            }
                        }
                    }
                    // Up
                    3 => {
                        for _ in 0..ins {
                            if pos_y >= 1 {
                                let ch = &map[pos_y - 1][pos_x];
                                if *ch == '.' {
                                    pos_y -= 1;
                                    continue;
                                } else if *ch == '#' {
                                    break;
                                }
                            }
                            // Wrap
                            let wrap_to = map[pos_y..].iter().enumerate().find_map(|(idx, ch)| {
                                match ch[pos_x] {
                                    ' ' => Some(idx + pos_y - 1),
                                    _ => None,
                                }
                            }).unwrap_or(map.len()-1);
                            if map[wrap_to][pos_x] != '#' {
                                pos_y = wrap_to;
                            } else {
                                break;
                            }
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
        rotate = !rotate;
    }

    println!("{}", (pos_y + 1) * 1000 + (pos_x + 1) * 4 + facing as usize);
}

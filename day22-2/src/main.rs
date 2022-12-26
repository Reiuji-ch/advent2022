use std::io::BufRead;
use std::str::FromStr;

// Length of the sides
const CUBE_DIM: usize = 50;

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

    // Parse the sides and number them
    let mut sides = Vec::new();
    for y in 0..map.len() / CUBE_DIM {
        for x in 0..max_width / CUBE_DIM {
            if map[y * CUBE_DIM][x * CUBE_DIM] != ' ' {
                sides.push((sides.len(), y * CUBE_DIM, x * CUBE_DIM));
            }
        }
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
    }).unwrap() as isize;
    let mut pos_y = 0isize;
    let mut facing = 0;
    for ins in instructions {
        match rotate {
            true => {
                facing = (facing + ins).rem_euclid(4);
            }
            false => {
                for _ in 0..ins {
                    let (new_x, new_y, new_facing) = wrap_or_move(pos_x, pos_y, facing);
                    if map[new_y as usize][new_x as usize] != '#' {
                        pos_y = new_y;
                        pos_x = new_x;
                        facing = new_facing;
                    } else {
                        break;
                    }
                }
            }
        }
        rotate = !rotate;
    }

    println!("{}", (pos_y + 1) * 1000 + (pos_x + 1) * 4 + facing);
}

// Move in the given direction, wrapping around as needed
// Hardcoded to my puzzle input
// My puzzle layout is as follows, with sides numbered
//  01
//  2
// 34
// 5
// VERY handheld, some of the comments and possibly some of the checks are wrong/out of date
// Only works for real puzzle input, provided as input.txt in this folder
pub fn wrap_or_move(from_x: isize, from_y: isize, facing: isize) -> (isize, isize, isize) {
    // Sanity check
    if from_x < 49 && from_y < 99 {
        panic!("OOB");
    }
    if from_x > 99 && from_y > 49 {
        panic!("OOB");
    }
    if from_x > 49 && from_y > 149 {
        panic!("OOB");
    }

    let (target_x, target_y) = match facing {
        0 => (from_x + 1, from_y),
        1 => (from_x, from_y + 1),
        2 => (from_x - 1, from_y),
        3 => (from_x, from_y - 1),
        _ => unreachable!(),
    };

    if target_y == -1 && target_x > 49 && target_x <= 99 && facing == 3 {
        // side '0' moving up, wraps to '5'
        (0, target_x + 100, 0)
    } else if target_x == 49 && target_y >= 0 && target_y <= 49 && facing == 2 {
        // side '0' moving left, wraps to '3'
        (0, 149 - target_y, 0)
    } else if target_x >= 100 && target_y == 50 && facing == 1 {
        // side '1' moving down, wraps to '2'
        (99, target_x - 50, 2)
    } else if target_x >= 100 && target_y == -1 && facing == 3 {
        // side '1' moving up, wraps to '5'
        (target_x - 100, 199, 3)
    } else if target_x >= 150 && target_y <= 49 && facing == 0 {
        // side '1' moving right, wraps to '3'
        (99, 149 - target_y, 2)
    } else if target_x == 49 && target_y >= 50 && target_y < 100 && facing == 2 {
        // side '2' moving left, wraps to '4'
        (target_y - 50, 100, 1)
    } else if target_x == 100 && target_y >= 50 && target_y < 100 && facing == 0 {
        // side '2' moving right, wraps to '1'
        (target_y + 50, 49, 3)
    } else if target_x == 100 && target_y >= 100 && target_y < 150 && facing == 0 {
        // side '3' moving right, wraps to '1'
        (149, 149 - target_y, 2)
    } else if target_x >= 50 && target_x < 100 && target_y == 150 && facing == 1 {
        // side '3' moving down, wraps to '5'
        (49, target_x + 100, 2)
    } else if target_x < 50 && target_y < 100 && facing == 3 {
        // side '4' moving up, wraps to '2'
        (50, target_x + 50, 0)
    } else if target_x == -1 && target_y >= 100 && target_y < 150 && facing == 2 {
        // side '4' moving left, wraps to '0'
        (50, 149 - target_y, 0) // ??
    } else if target_x == -1 && target_y >= 150 && facing == 2 {
        // side '5' moving left, wraps to '0'
        (target_y - 100, 0, 1)
    } else if target_x == 50 && target_y >= 150 && facing == 0 {
        // side '6' moving right, wraps to '0'
        (target_y - 100, 149, 3)
    } else if target_y >= 200 && facing == 1 {
        // side '6' moving down, wraps to '0'
        (target_x + 100, 0, 1)
    } else {
        (target_x, target_y, facing)
    }
}
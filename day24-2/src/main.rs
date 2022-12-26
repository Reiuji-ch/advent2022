use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut map_buffer = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                map_buffer.push(line);
            }
        }
    }

    let mut map_original = Vec::new();
    let mut blizzards = Vec::new();
    let mut y = 0;
    let mut x = 0;
    for line in map_buffer {
        let mut buf = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => {
                    buf.push(ch);
                }
                '.' => {
                    buf.push(ch);
                }
                _ => {
                    blizzards.push((y, x, ch));
                    buf.push('.');
                }
            }
            x += 1;
        }
        map_original.push(buf);
        x = 0;
        y += 1;
    }
    let start_x = map_original[0].iter().enumerate().find_map(|elem| match elem.1 {
        '.' => {
            Some(elem.0)
        }
        _ => {
            None
        }
    }).unwrap();
    let start_y = 0;
    let destination_x = map_original[map_original.len() - 1].iter().enumerate().find_map(|elem| match elem.1 {
        '.' => {
            Some(elem.0)
        }
        _ => {
            None
        }
    }).unwrap();
    let destination_y = map_original.len() - 1;

    let mut count1 = 1;
    let mut map = map_original.clone();
    map[start_y][start_x] = '+';
    while map[destination_y][destination_x] != '+' {
        for i in 0..blizzards.len() {
            match blizzards[i].2 {
                '>' => {
                    blizzards[i].1 = blizzards[i].1 + 1;
                    if blizzards[i].1 >= map[0].len() - 1 {
                        blizzards[i].1 = 1;
                    }
                }
                'v' => {
                    blizzards[i].0 = blizzards[i].0 + 1;
                    if blizzards[i].0 >= map.len() - 1 {
                        blizzards[i].0 = 1;
                    }
                }
                '<' => {
                    blizzards[i].1 = blizzards[i].1 - 1;
                    if blizzards[i].1 <= 0 {
                        blizzards[i].1 = map[0].len() - 2;
                    }
                }
                '^' => {
                    blizzards[i].0 = blizzards[i].0 - 1;
                    if blizzards[i].0 <= 0 {
                        blizzards[i].0 = map.len() - 2;
                    }
                }
                _ => unreachable!()
            };
        }

        let mut map_clone = map.clone();
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                map_clone[y][x] = '.';
            }
        }
        for blizzard in &blizzards {
            map_clone[blizzard.0][blizzard.1] = '#';
        }
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                if map_clone[y][x] == '.' {
                    let can_wait = map[y][x] == '+';
                    let can_left = map[y][x+1] == '+';
                    let can_right = map[y][x-1] == '+';
                    let can_up = map[y-1][x] == '+';
                    let can_down = map[y+1][x] == '+';
                    if can_wait || can_left || can_right || can_down || can_up {
                        map_clone[y][x] = '+';
                    }
                }
            }
        }
        if map_clone[destination_y-1][destination_x] == '+' {
            map_clone[destination_y][destination_x] = '+';
        }

        /*
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                eprint!("{}", map_clone[y][x]);
            }
            eprintln!();
        }
        */
        map = map_clone;
        //std::thread::sleep(Duration::from_secs(1));
        count1 += 1;
    }

    let mut count2 = 0;
    let mut map = map_original.clone();
    map[destination_y][destination_x] = '+';
    while map[start_y][start_x] != '+' {
        for i in 0..blizzards.len() {
            match blizzards[i].2 {
                '>' => {
                    blizzards[i].1 = blizzards[i].1 + 1;
                    if blizzards[i].1 >= map[0].len() - 1 {
                        blizzards[i].1 = 1;
                    }
                }
                'v' => {
                    blizzards[i].0 = blizzards[i].0 + 1;
                    if blizzards[i].0 >= map.len() - 1 {
                        blizzards[i].0 = 1;
                    }
                }
                '<' => {
                    blizzards[i].1 = blizzards[i].1 - 1;
                    if blizzards[i].1 <= 0 {
                        blizzards[i].1 = map[0].len() - 2;
                    }
                }
                '^' => {
                    blizzards[i].0 = blizzards[i].0 - 1;
                    if blizzards[i].0 <= 0 {
                        blizzards[i].0 = map.len() - 2;
                    }
                }
                _ => unreachable!()
            };
        }

        let mut map_clone = map.clone();
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                map_clone[y][x] = '.';
            }
        }
        for blizzard in &blizzards {
            map_clone[blizzard.0][blizzard.1] = '#';
        }
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                if map_clone[y][x] == '.' {
                    let can_wait = map[y][x] == '+';
                    let can_left = map[y][x+1] == '+';
                    let can_right = map[y][x-1] == '+';
                    let can_up = map[y-1][x] == '+';
                    let can_down = map[y+1][x] == '+';
                    if can_wait || can_left || can_right || can_down || can_up {
                        map_clone[y][x] = '+';
                    }
                }
            }
        }
        if map_clone[start_y+1][start_x] == '+' {
            map_clone[start_y][start_x] = '+';
        }

        /*
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                eprint!("{}", map_clone[y][x]);
            }
            eprintln!();
        }
        */
        map = map_clone;
        //std::thread::sleep(Duration::from_secs(1));
        count2 += 1;
    }

    let mut count3 = 0;
    let mut map = map_original.clone();
    map[start_y][start_x] = '+';
    while map[destination_y][destination_x] != '+' {
        for i in 0..blizzards.len() {
            match blizzards[i].2 {
                '>' => {
                    blizzards[i].1 = blizzards[i].1 + 1;
                    if blizzards[i].1 >= map[0].len() - 1 {
                        blizzards[i].1 = 1;
                    }
                }
                'v' => {
                    blizzards[i].0 = blizzards[i].0 + 1;
                    if blizzards[i].0 >= map.len() - 1 {
                        blizzards[i].0 = 1;
                    }
                }
                '<' => {
                    blizzards[i].1 = blizzards[i].1 - 1;
                    if blizzards[i].1 <= 0 {
                        blizzards[i].1 = map[0].len() - 2;
                    }
                }
                '^' => {
                    blizzards[i].0 = blizzards[i].0 - 1;
                    if blizzards[i].0 <= 0 {
                        blizzards[i].0 = map.len() - 2;
                    }
                }
                _ => unreachable!()
            };
        }

        let mut map_clone = map.clone();
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                map_clone[y][x] = '.';
            }
        }
        for blizzard in &blizzards {
            map_clone[blizzard.0][blizzard.1] = '#';
        }
        for y in 1..map_clone.len() - 1 {
            for x in 1..map_clone[0].len() - 1 {
                if map_clone[y][x] == '.' {
                    let can_wait = map[y][x] == '+';
                    let can_left = map[y][x+1] == '+';
                    let can_right = map[y][x-1] == '+';
                    let can_up = map[y-1][x] == '+';
                    let can_down = map[y+1][x] == '+';
                    if can_wait || can_left || can_right || can_down || can_up {
                        map_clone[y][x] = '+';
                    }
                }
            }
        }
        if map_clone[destination_y-1][destination_x] == '+' {
            map_clone[destination_y][destination_x] = '+';
        }

        /*
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                eprint!("{}", map_clone[y][x]);
            }
            eprintln!();
        }
        */
        map = map_clone;
        //std::thread::sleep(Duration::from_secs(1));
        count3 += 1;
    }

    //eprintln!("{map:?}");
    //eprintln!("{blizzards:?}");

    println!("{}", count1 + count2 + count3);
}

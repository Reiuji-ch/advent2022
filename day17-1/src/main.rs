use std::io::BufRead;

pub fn main() {
    let input = std::io::stdin().lock();

    let moves: Vec<isize> = input.lines().next().unwrap().unwrap().chars().map(|ch| {
        match ch {
            '<' => -1isize,
            '>' => 1isize,
            _ => unreachable!(),
        }
    }).collect();

    let mut move_count = 0;
    let mut highest = 0;
    let (mut x, mut y) = (2usize, 3usize);
    let mut tower = vec![[false; 7]; 10000];
    for rock in 0..2022 {
        let piece_width = match rock % 5 {
            0 => 4,
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => unreachable!(),
        };

        loop {
            // Check if we can move this way, limited by the walls
            let destination_x = match moves[move_count % moves.len()] {
                -1 => {
                    if x > 0 {
                        x - 1
                    } else {
                        0
                    }
                },
                1 => {
                    if x+1 <= 7-piece_width {
                        x + 1
                    } else {
                        x
                    }
                }
                _ => unreachable!(),
            };
            // Check if we can move this way, limited by existing settled pieces
            let can_move_x = destination_x != x && match rock % 5 {
                0 => {
                    destination_x <= 3
                        && tower[y][destination_x] == false
                        && tower[y][destination_x+1] == false
                        && tower[y][destination_x+2] == false
                        && tower[y][destination_x+3] == false
                }
                1 => {
                    destination_x <= 4
                        && tower[y+1][destination_x] == false
                        && tower[y][destination_x+1] == false
                        && tower[y+1][destination_x+1] == false
                        && tower[y+1][destination_x+2] == false
                        && tower[y+2][destination_x+1] == false
                }
                2 => {
                    destination_x <= 4
                        && tower[y][destination_x] == false
                        && tower[y][destination_x+1] == false
                        && tower[y][destination_x+2] == false
                        && tower[y+1][destination_x+2] == false
                        && tower[y+2][destination_x+2] == false
                }
                3 => {
                    tower[y][destination_x] == false
                        && tower[y+1][destination_x] == false
                        && tower[y+2][destination_x] == false
                        && tower[y+3][destination_x] == false
                }
                4 => {
                    destination_x <= 5
                        && tower[y][destination_x] == false
                        && tower[y][destination_x+1] == false
                        && tower[y+1][destination_x+1] == false
                        && tower[y+1][destination_x] == false
                }
                _ => unreachable!(),
            };
            if can_move_x {
                x = destination_x;
            }

            // Try to move down
            let can_move_y = y >= 1 && match rock % 5 {
                0 => {
                        tower[y-1][x] == false
                        && tower[y-1][x+1] == false
                        && tower[y-1][x+2] == false
                        && tower[y-1][x+3] == false
                }
                1 => {
                    tower[y][x] == false
                        && tower[y-1][x+1] == false
                        && tower[y][x+2] == false
                }
                2 => {
                    tower[y-1][x] == false
                        && tower[y-1][x+1] == false
                        && tower[y-1][x+2] == false
                }
                3 => {
                    tower[y-1][x] == false
                }
                4 => {
                    tower[y-1][x] == false
                        && tower[y-1][x+1] == false
                }
                _ => unreachable!(),
            };
            if can_move_y {
                y -= 1;
            } else {
                match rock % 5 {
                    0 => {
                        tower[y][x] = true;
                        tower[y][x+1] = true;
                        tower[y][x+2] = true;
                        tower[y][x+3] = true;
                        highest = highest.max(y);
                    }
                    1 => {
                        tower[y][x+1] = true;
                        tower[y+1][x] = true;
                        tower[y+1][x+1] = true;
                        tower[y+1][x+2] = true;
                        tower[y+2][x+1] = true;
                        highest = highest.max(y+2);
                    }
                    2 => {
                        tower[y][x] = true;
                        tower[y][x+1] = true;
                        tower[y][x+2] = true;
                        tower[y+1][x+2] = true;
                        tower[y+2][x+2] = true;
                        highest = highest.max(y+2);
                    }
                    3 => {
                        tower[y][x] = true;
                        tower[y+1][x] = true;
                        tower[y+2][x] = true;
                        tower[y+3][x] = true;
                        highest = highest.max(y+3);
                    }
                    4 => {
                        tower[y][x] = true;
                        tower[y+1][x] = true;
                        tower[y][x+1] = true;
                        tower[y+1][x+1] = true;
                        highest = highest.max(y+1);
                    }
                    _ => unreachable!(),
                };

                x = 2;
                y = highest+4;
                move_count += 1;
                break;
            }
            move_count += 1;
        }
    }

    // 0-indexed height, add 1 before outputting
    println!("{}", highest+1);
}
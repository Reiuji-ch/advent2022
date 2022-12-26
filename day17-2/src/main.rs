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

    // Memorize seen pairs of rocks and moves
    // We will also memorize the furthest any one piece fell and the amount of rocks
    // [rock_type][move_count % moves.len()]
    // At each position, we store (highest, rocks_used, max_fall)
    // Whenever a rock comes to a rest, we check if we've already seen this combination of rock and move number
    // If we have seen it and the max_fall is the same as seen before, we check if there is a cycle
    // The idea is, if we see a potential cycle and the max fall is the same, then no piece can have fallen further than max_fall
    // That must means that everything below `highest-max_fall` is never touched by subsequent pieces
    // We then test if we do have a cycle by comparing the the current potential cycle with the one immediately below it
    // If they are the exact same, max fall, rock usage etc. checks out, we can compute how many loops there are
    // We then simply compute `highest + cycle_height * loop_count` to get the final height
    let mut seen_combos = vec![vec![(0,0,99999); moves.len()]; 5];
    let mut max_fall_dist;
    let mut total_max_fall = 0;

    let mut move_count = 0;
    let mut highest = 0;
    let (mut x, mut y) = (2usize, 3usize);
    // This buffer just needs to be "Big Enough™" to detect a cycle
    // The cycle _probably always_ happens relatively early (<10000 rocks), so this should be fine
    let mut tower = vec![[false; 7]; 1000000];
    // This loop shouldn't even run to completion, since a valid cycle breaks it
    for rock in 0..1000000000000usize {
        max_fall_dist = 0;
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
                max_fall_dist += 1;
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
                }
                move_count += 1;

                total_max_fall = total_max_fall.max(max_fall_dist);

                // We placed a piece, check if there is a cycle
                // Check if this combination of (rock_type, move_number) is already known
                // Check max fall distance is same as max known
                if seen_combos[rock % 5][move_count % moves.len()].0 != 0
                    && seen_combos[rock % 5][move_count % moves.len()].2 == total_max_fall {
                    // Read the previous cycles data
                    let (start, rock_count, max_fall) = seen_combos[rock % 5][move_count % moves.len()];
                    // Compute the height of the cycle
                    let height = highest - start;
                    // Compute where the cycle starts. Since we skip max_fall pieces, it is effectively `highest-height-max_fall`
                    let check_start = start-max_fall;
                    // Ensure there is actually a full cycle below us
                    if check_start >= height {
                        // Check previous cycle and current is the same
                        // This checks if each cell in each row are identical across the cycles
                        let mut same = true;
                        for n in check_start..check_start + height {
                            for asd in 0..7 {
                                if tower[n][asd] != tower[n - height][asd] {
                                    same = false;
                                    break;
                                }
                            }
                            if !same {
                                break;
                            }
                        }
                        // If they appear to be the same, verify the rocks used checks out
                        if same {
                            let rocks_used = rock-rock_count;
                            let reps = (1000000000000usize - rock)/rocks_used;
                            // If the detection is not at a point where we'd have an integer number of cycles, discard it
                            // If it is, compute the total height and exit
                            if (1000000000000usize - rock) % rocks_used == 0 {
                                println!("{}", height*reps + highest);
                                std::process::exit(0);
                            }
                        }
                    }
                }
                // Record seen data
                seen_combos[rock % 5][move_count % moves.len()] = (highest, rock, total_max_fall);

                x = 2;
                y = highest+4;
                break;
            }
            move_count += 1;
        }
    }
}
use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    // Layout is row-major, [y][x] indexed
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let mut row = Vec::new();
                for ch in line.chars() {
                    row.push(ch.to_digit(10).expect("Invalid number") as u8);
                }
                grid.push(row);
            }
        }
    }

    let mut best_score = 0;
    // Find score for each tree, keep the best one
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            best_score = best_score.max(check_score(&grid, x, y, grid[y][x]));
        }
    }

    println!("{best_score}");
}

fn check_score(grid: &Vec<Vec<u8>>, x: usize, y: usize, height: u8) -> usize {
    let mut y1_score = 0;
    let mut y2_score = 0;
    let mut x1_score = 0;
    let mut x2_score = 0;
    for y1 in (0..y).rev() {
        y1_score += 1;
        if grid[y1][x] >= height {
            break;
        }
    }
    for y2 in y+1..grid.len() {
        y2_score += 1;
        if grid[y2][x] >= height {
            break;
        }
    }
    for x1 in (0..x).rev() {
        x1_score += 1;
        if grid[y][x1] >= height {
            break;
        }
    }
    for x2 in x+1..grid[y].len() {
        x2_score += 1;
        if grid[y][x2] >= height {
            break;
        }
    }

    return y1_score * y2_score * x1_score * x2_score;
}
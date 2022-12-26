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

    let mut visible = 0;
    // All trees are visible, add them to total
    visible += grid.len() * 2;
    visible += grid[0].len() * 2 - 4;
    // For each inner tree, check if it is visible
    for y in 1..grid.len()-1 {
        for x in 1..grid.len()-1 {
            visible += check_height(&grid, x, y, grid[y][x]);
        }
    }

    println!("{visible}");
}

fn check_height(grid: &Vec<Vec<u8>>, x: usize, y: usize, height: u8) -> usize {
    let mut y1_hidden = false;
    let mut y2_hidden = false;
    let mut x1_hidden = false;
    let mut x2_hidden = false;
    for y1 in 0..y {
        if grid[y1][x] >= height {
            y1_hidden = true;
            break;
        }
    }
    for y2 in y+1..grid.len() {
        if grid[y2][x] >= height {
            y2_hidden = true;
            break;
        }
    }
    for x1 in 0..x {
        if grid[y][x1] >= height {
            x1_hidden = true;
            break;
        }
    }
    for x2 in x+1..grid[y].len() {
        if grid[y][x2] >= height {
            x2_hidden = true;
            break;
        }
    }

    // If it is hidden from all directions, return 0
    // If it is visible from ANY direction, return 1
    match y1_hidden && y2_hidden && x1_hidden && x2_hidden {
        true => 0,
        false => 1,
    }
}
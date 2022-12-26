use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut cubes_pos = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let mut split = line.split(",");
                cubes_pos.push((usize::from_str(split.next().unwrap()).unwrap(),
                            usize::from_str(split.next().unwrap()).unwrap(),
                            usize::from_str(split.next().unwrap()).unwrap()));
            }
        }
    }
    let max_x = cubes_pos.iter().fold(usize::MIN, |acc, elem| acc.max(elem.0)) + 1;
    let max_y = cubes_pos.iter().fold(usize::MIN, |acc, elem| acc.max(elem.1)) + 1;
    let max_z = cubes_pos.iter().fold(usize::MIN, |acc, elem| acc.max(elem.2)) + 1;

    eprintln!("{max_x},{max_y},{max_z}");

    let mut grid = vec![vec![vec![false; max_z]; max_y]; max_x];
    for cube in cubes_pos {
        let (x,y,z) = cube;
        grid[x][y][z] = true;
    }

    let mut surface_area = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for z in 0..grid[x][y].len() {
                if grid[x][y][z] == false {
                    continue;
                }
                let mut area = 6;
                if x > 0 {
                    if grid[x-1][y][z] == true {
                        area -= 1;
                    }
                }
                if x < grid.len()-1 {
                    if grid[x+1][y][z] == true {
                        area -= 1;
                    }
                }

                if y > 0 {
                    if grid[x][y-1][z] == true {
                        area -= 1;
                    }
                }
                if y < grid[x].len()-1 {
                    if grid[x][y+1][z] == true {
                        area -= 1;
                    }
                }

                if z > 0 {
                    if grid[x][y][z-1] == true {
                        area -= 1;
                    }
                }
                if z < grid[x][y].len()-1 {
                    if grid[x][y][z+1] == true {
                        area -= 1;
                    }
                }
                surface_area += area;
            }
        }
    }

    println!("{surface_area}");
}
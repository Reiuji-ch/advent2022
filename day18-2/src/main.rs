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

    let mut grid = vec![vec![vec![None; max_z]; max_y]; max_x];
    for cube in cubes_pos {
        let (x, y, z) = cube;
        grid[x][y][z] = Some(0);
    }

    let mut visited = vec![vec![vec![false; max_z]; max_y]; max_x];
    let mut queue: Vec<(usize, usize, usize)> = Vec::new();

    // Set the sides of edges facing "outside" of the cube and queue those spots
    for x in 0..max_x {
        for y in 0..max_y {
            if let Some(val) = grid[x][y][0] {
                grid[x][y][0] = Some(val | 16);
            }
            if let Some(val) = grid[x][y][max_z-1] {
                grid[x][y][max_z-1] = Some(val | 32);
            }
            if !visited[x][y][0] {
                queue.push((x,y,0));
                visited[x][y][0] = true;
            }
            if !visited[x][y][max_z-1] {
                queue.push((x, y, max_z - 1));
                visited[x][y][max_z-1] = true;
            }
        }
    }
    for x in 0..max_x {
        for z in 0..max_z {
            if let Some(val) = grid[x][0][z] {
                grid[x][0][z] = Some(val | 4);
            }
            if let Some(val) = grid[x][max_y-1][z] {
                grid[x][max_y-1][z] = Some(val | 8);
            }
            if !visited[x][0][z] {
                queue.push((x,0,z));
                visited[x][0][z] = true;
            }
            if !visited[x][max_y-1][z] {
                queue.push((x,max_y-1,z));
                visited[x][max_y-1][z] = true;
            }
        }
    }
    for y in 0..max_y {
        for z in 0..max_z {
            if let Some(val) = grid[0][y][z] {
                grid[0][y][z] = Some(val | 1);
            }
            if let Some(val) = grid[max_x-1][y][z] {
                grid[max_x-1][y][z] = Some(val | 2);
            }
            if !visited[0][y][z] {
                queue.push((0,y,z));
                visited[0][y][z] = true;
            }
            if !visited[max_x-1][y][z] {
                queue.push((max_x-1,y,z));
                visited[max_x-1][y][z] = true;
            }
        }
    }

    while let Some(coord) = queue.pop() {
        let (x, y, z) = coord;
        // Ignore cells that are solid
        if grid[x][y][z].is_some() {
            continue;
        }
        if x > 0 {
            if let Some(cell) = grid[x - 1][y][z] {
                grid[x - 1][y][z] = Some(cell | 2);
            } else if !visited[x - 1][y][z] {
                visited[x - 1][y][z] = true;
                queue.push((x - 1, y, z));
            }
        }
        if x < grid.len() - 1 {
            if let Some(cell) = grid[x + 1][y][z] {
                grid[x + 1][y][z] = Some(cell | 1);
            } else if !visited[x + 1][y][z] {
                visited[x + 1][y][z] = true;
                queue.push((x + 1, y, z));
            }
        }

        if y > 0 {
            if let Some(cell) = grid[x][y - 1][z] {
                grid[x][y - 1][z] = Some(cell | 8);
            } else if !visited[x][y - 1][z] {
                visited[x][y - 1][z] = true;
                queue.push((x, y - 1, z));
            }
        }
        if y < grid[x].len() - 1 {
            if let Some(cell) = grid[x][y + 1][z] {
                grid[x][y + 1][z] = Some(cell | 4);
            } else if !visited[x][y + 1][z] {
                visited[x][y + 1][z] = true;
                queue.push((x, y + 1, z));
            }
        }

        if z > 0 {
            if let Some(cell) = grid[x][y][z - 1] {
                grid[x][y][z - 1] = Some(cell | 32);
            } else if !visited[x][y][z - 1] {
                visited[x][y][z - 1] = true;
                queue.push((x, y, z - 1));
            }
        }
        if z < grid[x][y].len() - 1 {
            if let Some(cell) = grid[x][y][z + 1] {
                grid[x][y][z + 1] = Some(cell | 16);
            } else if !visited[x][y][z + 1] {
                visited[x][y][z + 1] = true;
                queue.push((x, y, z + 1));
            }
        }
    }

    let mut surface_area = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for z in 0..grid[x][y].len() {
                if let Some(sides) = grid[x][y][z] {
                    if sides & 1 > 0 {
                        surface_area += 1;
                    }
                    if sides & 2 > 0 {
                        surface_area += 1;
                    }
                    if sides & 4 > 0 {
                        surface_area += 1;
                    }
                    if sides & 8 > 0 {
                        surface_area += 1;
                    }
                    if sides & 16 > 0 {
                        surface_area += 1;
                    }
                    if sides & 32 > 0 {
                        surface_area += 1;
                    }
                }
            }
        }
    }

    println!("{surface_area}");
}
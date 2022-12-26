use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut rocks = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let segments = line.split(" -> ").map(|coordinates| {
                    let mut iter = coordinates.split(",").map(|n| usize::from_str(n).unwrap());
                    (iter.next().unwrap(), iter.next().unwrap())
                }).collect::<Vec<(usize, usize)>>();
                rocks.push(segments);
            }
        }
    }
    let min_x = rocks.iter().fold(usize::MAX, |min, rock| {
        min.min(rock.iter().fold(usize::MAX, |min, coord| min.min(coord.0)))
    });
    let max_x = rocks.iter().fold(0, |max, rock| {
        max.max(rock.iter().fold(0, |max, coord| max.max(coord.0)))
    });
    let max_y = rocks.iter().fold(0, |max, rock| {
        max.max(rock.iter().fold(0, |max, coord| max.max(coord.1)))
    });

    // 0-indexed coordinates, so we need to add 1 capacity
    // Additionally, we allocate an additional 'gutter' area that serves as the void on the left, right and bottom sides
    let width = (max_x - min_x) + 3;
    let height = max_y + 2;
    let left_idx = min_x - 1;

    let mut cave = vec![vec![Material::Air; width]; height];
    for rock in &rocks {
        for i in 0..rock.len() - 1 {
            let left = rock[i].0.min(rock[i + 1].0) - left_idx;
            let right = rock[i].0.max(rock[i + 1].0) - left_idx;
            let up = rock[i].1.min(rock[i + 1].1);
            let down = rock[i].1.max(rock[i + 1].1);

            for x in left..right + 1 {
                for y in up..down + 1 {
                    cave[y][x] = Material::Rock;
                }
            }
        }
    }

    let mut resting_sand = 0;
    let mut sand_x = 500 - left_idx;
    let mut sand_y = 0;
    loop {
        if sand_y >= max_y || sand_x == 0 || sand_x == width-1 {
            break;
        }
        match cave[sand_y + 1][sand_x] {
            Material::Air => {
                sand_y += 1;
                continue;
            }
            Material::Rock => {}
            Material::Sand => {}
        };
        match cave[sand_y + 1][sand_x - 1] {
            Material::Air => {
                sand_y += 1;
                sand_x -= 1;
                continue;
            }
            Material::Rock => {}
            Material::Sand => {}
        };
        match cave[sand_y + 1][sand_x + 1] {
            Material::Air => {
                sand_y += 1;
                sand_x += 1;
                continue;
            }
            Material::Rock => {}
            Material::Sand => {}
        };
        resting_sand += 1;
        cave[sand_y][sand_x] = Material::Sand;
        sand_x = 500 - left_idx;
        sand_y = 0;
    }

    println!("{resting_sand}");
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Material {
    Air,
    Rock,
    Sand,
}
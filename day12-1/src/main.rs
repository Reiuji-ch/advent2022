use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();

    let mut map: Vec<Vec<u8>> = Vec::new();
    let (mut current_x, mut current_y) = (0, 0);
    let (mut goal_x, mut goal_y) = (0, 0);

    let mut row = 0;
    let mut col;
    for line in input.lines() {
        let line = line.expect("Read error");
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                col = 0;
                let mut line_buf = Vec::new();
                for ch in line.chars() {
                    match ch {
                        'S' => {
                            current_x = col;
                            current_y = row;
                            line_buf.push('a' as u8 - 97);
                        }
                        'E' => {
                            goal_x = col;
                            goal_y = row;
                            line_buf.push('z' as u8 - 97);
                        }
                        _ => {
                            line_buf.push(ch as u8 - 97);
                        }
                    }
                    col += 1;
                }
                row += 1;
                map.push(line_buf);
            }
        }
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; map[0].len()]; map.len()];
    distances[current_y][current_x] = 0;
    let mut queue = BinaryHeap::new();

    queue.push(Coordinate {
        priority: usize::MAX,
        position: (current_y, current_x)
    });
    while queue.peek().is_some() {
        (current_y, current_x) = queue.pop().expect("Ran out of paths before reaching goal").position;
        let current_elevation = map[current_y][current_x];
        let current_distance = distances[current_y][current_x];
        if current_x == goal_x && current_y == goal_y {
            println!("{current_distance}");
            break;
        }
        if current_x > 0 {
            if can_reach(current_elevation, map[current_y][current_x-1]) && !visited[current_y][current_x-1] {
                if queue.iter().find(|x| x.position == (current_y, current_x - 1)).is_none() {
                    queue.push(Coordinate {
                        priority: usize::MAX - current_distance,
                        position: (current_y, current_x - 1)
                    });
                }
                distances[current_y][current_x-1] = distances[current_y][current_x-1].min(current_distance + 1);
            }
        }
        if current_y > 0 {
            if can_reach(current_elevation, map[current_y-1][current_x]) && !visited[current_y-1][current_x] {
                if queue.iter().find(|x| x.position == (current_y - 1, current_x)).is_none() {
                    queue.push(Coordinate {
                        priority: usize::MAX - current_distance,
                        position: (current_y - 1, current_x)
                    });
                }
                distances[current_y-1][current_x] = distances[current_y-1][current_x].min(current_distance + 1);
            }
        }
        if current_x < map[0].len() - 1 {
            if can_reach(current_elevation, map[current_y][current_x+1]) && !visited[current_y][current_x+1] {
                if queue.iter().find(|x| x.position == (current_y, current_x + 1)).is_none() {
                    queue.push(Coordinate {
                        priority: usize::MAX - current_distance,
                        position: (current_y, current_x + 1)
                    });
                }
                distances[current_y][current_x+1] = distances[current_y][current_x+1].min(current_distance + 1);
            }
        }
        if current_y < map.len() - 1 {
            if can_reach(current_elevation, map[current_y+1][current_x]) && !visited[current_y+1][current_x] {
                if queue.iter().find(|x| x.position == (current_y + 1, current_x)).is_none() {
                    queue.push(Coordinate {
                        priority: usize::MAX - current_distance,
                        position: (current_y + 1, current_x)
                    });
                }
                distances[current_y+1][current_x] = distances[current_y+1][current_x].min(current_distance + 1);
            }
        }
        visited[current_y][current_x] = true;
    }
}

fn can_reach(from: u8, to: u8) -> bool {
    from + 1 >= to
}

#[derive(Copy, Clone, Eq, PartialOrd, PartialEq)]
struct Coordinate {
    priority: usize,
    position: (usize, usize),
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}
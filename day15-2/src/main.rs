use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut sensors = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let sensor_x = isize::from_str(
                    &line[line.find("x=").unwrap()+2..line.find(",").unwrap()]
                ).unwrap();
                let sensor_y = isize::from_str(
                    &line[line.find("y=").unwrap()+2..line.find(":").unwrap()]
                ).unwrap();

                let beacon_x = isize::from_str(
                    &line[line.rfind("x=").unwrap()+2..line.rfind(",").unwrap()]
                ).unwrap();
                let beacon_y = isize::from_str(
                    &line[line.rfind("y=").unwrap()+2..]
                ).unwrap();

                let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);

                sensors.push(Sensor {
                    x: sensor_x,
                    y: sensor_y,
                    beacon_x,
                    beacon_y,
                    dist: distance as isize
                });
            }
        }
    }

    // Diagonals by where they would be at x=0 and whether they go up (true) or down (false)
    let mut diagonals: Vec<(isize, bool)> = Vec::new();
    for sensor in &sensors {
        // Add the four diagonals
        diagonals.push((sensor.y - (sensor.x-sensor.dist-1), false));
        diagonals.push((sensor.y + (sensor.x-sensor.dist-1), true));

        diagonals.push((sensor.y - (sensor.x+sensor.dist+1), false));
        diagonals.push((sensor.y + (sensor.x+sensor.dist+1), true));
    }

    let limit = 4000000;
    let mut intersects = HashMap::with_capacity(10000);
    for i in 0..diagonals.len() {
        let diag1 = &diagonals[i];
        if diag1.1 != true {
            continue;
        }
        for j in 0..diagonals.len() {
            let diag2 = &diagonals[j];
            if diag2.1 != false {
                continue;
            }
            let x = (diag2.0 - diag1.0) / -2;
            let coord = (x, diag1.0 - x);
            if coord.0 >= 0 && coord.0 <= limit && coord.1 >= 0 && coord.1 <= limit {
                intersects.insert((x, diag1.0 - x), intersects.get(&(x, diag1.0 - x)).unwrap_or(&0) + 1);
            }
        }
    }

    let mut intersects: Vec<((isize, isize), usize)> = intersects.into_iter().collect();
    intersects.sort_unstable_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    intersects.dedup();

    loop {
        let elem = intersects.pop().unwrap().0;
        let mut valid = true;
        for sensor in &sensors {
            if (elem.0.abs_diff(sensor.x) + elem.1.abs_diff(sensor.y)) as isize <= sensor.dist {
                valid = false;
                break;
            }
        }
        if valid {
            println!("{}", elem.0*4000000 + elem.1);
            break;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Sensor {
    pub x: isize,
    pub y: isize,
    pub beacon_x: isize,
    pub beacon_y: isize,
    pub dist: isize,
}
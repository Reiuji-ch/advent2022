use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut sensors = Vec::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
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
                min_x = min_x.min(sensor_x - distance as isize);
                max_x = max_x.max(beacon_x + distance as isize);

                sensors.push(Sensor {
                    x: sensor_x,
                    y: sensor_y,
                    beacon_x,
                    beacon_y,
                    dist: distance
                });
            }
        }
    }

    let mut invalid = 0;
    for x in min_x..max_x+1 {
        for sensor in &sensors {
            let dist = x.abs_diff(sensor.x) + 2000000isize.abs_diff(sensor.y);
            // If dist is 0, we know there already is a beacon, so don't count it
            if dist <= sensor.dist && !(x == sensor.beacon_x && 2000000 == sensor.beacon_y) {
                invalid += 1;
                break;
            }
        }
    }

    println!("{invalid:?}");
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Sensor {
    pub x: isize,
    pub y: isize,
    pub beacon_x: isize,
    pub beacon_y: isize,
    pub dist: usize,
}
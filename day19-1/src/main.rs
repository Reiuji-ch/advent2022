use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut blueprints = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let idx = line.find("costs ").unwrap() + 6;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let ore = usize::from_str(&line[idx..idx_end]).unwrap();

                let idx = line[idx_end..].find("costs ").unwrap() + idx_end + 6;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let clay = usize::from_str(&line[idx..idx_end]).unwrap();

                let idx = line[idx_end..].find("costs ").unwrap() + idx_end + 6;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let obsidian_ore = usize::from_str(&line[idx..idx_end]).unwrap();
                let idx = line[idx_end..].find("and ").unwrap() + idx_end + 4;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let obsidian_clay = usize::from_str(&line[idx..idx_end]).unwrap();

                let idx = line[idx_end..].find("costs ").unwrap() + idx_end + 6;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let geode_ore = usize::from_str(&line[idx..idx_end]).unwrap();
                let idx = line[idx_end..].find("and ").unwrap() + idx_end + 4;
                let idx_end = line[idx..].find(" ").unwrap() + idx;
                let geode_obsidian = usize::from_str(&line[idx..idx_end]).unwrap();

                blueprints.push(Blueprint {
                    ore,
                    clay,
                    obsidian: (obsidian_ore, obsidian_clay),
                    geode: (geode_ore, geode_obsidian),
                });
            }
        }
    }

    let mut idx = 1;
    let mut quality = 0;
    for blueprint in blueprints {
        quality += recurse(&blueprint, (0, 1, 0, 0, 0, 0, 0, 0), 24) * idx;
        idx += 1;
    }

    println!("{quality}");
}

fn recurse(blueprint: &Blueprint, state: (usize, usize, usize, usize, usize, usize, usize, usize), mut turns: isize) -> usize {
    if turns <= 0 {
        return state.6;
    }

    let (mut ore, mut ore_prod, mut clay, mut clay_prod, mut obsidian, mut obsidian_prod, mut geode, mut geode_prod) = state;
    let mut best = geode;
    let turns_required = if ore >= blueprint.ore {
        0
    } else {
        if (blueprint.ore - ore) % ore_prod == 0 {
            (blueprint.ore - ore) / ore_prod
        } else {
            (blueprint.ore - ore) / ore_prod + 1
        }
    } + 1;
    if turns-turns_required as isize >= 0 && ore_prod < blueprint.ore.max(blueprint.clay).max(blueprint.obsidian.0).max(blueprint.geode.0) {
        best = best.max(recurse(blueprint,
                                (ore + turns_required * ore_prod - blueprint.ore, ore_prod + 1,
                                 clay + turns_required * clay_prod, clay_prod,
                                 obsidian + turns_required * obsidian_prod, obsidian_prod,
                                 geode + turns_required * geode_prod, geode_prod), turns - turns_required as isize));
    } else {
        best = best.max(geode + geode_prod * turns as usize);
    }

    let turns_required = if ore >= blueprint.clay {
        0
    } else {
        if (blueprint.clay - ore) % ore_prod == 0 {
            (blueprint.clay - ore) / ore_prod
        } else {
            (blueprint.clay - ore) / ore_prod + 1
        }
    } + 1;
    if turns-turns_required as isize >= 0 && clay_prod < blueprint.obsidian.1 {
        best = best.max(recurse(blueprint,
                                (ore + turns_required * ore_prod - blueprint.clay, ore_prod,
                                 clay + turns_required * clay_prod, clay_prod + 1,
                                 obsidian + turns_required * obsidian_prod, obsidian_prod,
                                 geode + turns_required * geode_prod, geode_prod), turns - turns_required as isize));
    } else {
        best = best.max(geode + geode_prod * turns as usize);
    }

    if clay_prod >= 1 {
        let turns_required = if ore >= blueprint.obsidian.0 && clay >= blueprint.obsidian.1 {
            0
        } else {
            let ore_turns = if ore >= blueprint.obsidian.0 {
                0
            } else if (blueprint.obsidian.0 - ore) % ore_prod == 0 {
                (blueprint.obsidian.0 - ore) / ore_prod
            } else {
                (blueprint.obsidian.0 - ore) / ore_prod + 1
            };
            let clay_turns = if clay >= blueprint.obsidian.1 {
                0
            } else if (blueprint.obsidian.1 - clay) % clay_prod == 0 {
                (blueprint.obsidian.1 - clay) / clay_prod
            } else {
                (blueprint.obsidian.1 - clay) / clay_prod + 1
            };
            ore_turns.max(clay_turns)
        } + 1;
        if turns-turns_required as isize >= 0 && obsidian_prod < blueprint.geode.1 {
            best = best.max(recurse(blueprint,
                                    (ore + turns_required * ore_prod - blueprint.obsidian.0, ore_prod,
                                     clay + turns_required * clay_prod - blueprint.obsidian.1, clay_prod,
                                     obsidian + turns_required * obsidian_prod, obsidian_prod + 1,
                                     geode + turns_required * geode_prod, geode_prod), turns - turns_required as isize));
        } else {
            best = best.max(geode + geode_prod * turns as usize);
        }
    }
    if obsidian_prod >= 1 {
        let turns_required = if ore >= blueprint.geode.0 && obsidian >= blueprint.geode.1 {
            0
        } else {
            let ore_turns = if ore > blueprint.geode.0 {
                0
            } else if (blueprint.geode.0 - ore) % ore_prod == 0 {
                (blueprint.geode.0 - ore) / ore_prod
            } else {
                (blueprint.geode.0 - ore) / ore_prod + 1
            };
            let obsidian_turns = if obsidian >= blueprint.geode.1 {
                0
            } else if (blueprint.geode.1 - obsidian) % obsidian_prod == 0 {
                (blueprint.geode.1 - obsidian) / obsidian_prod
            } else {
                (blueprint.geode.1 - obsidian) / obsidian_prod + 1
            };
            ore_turns.max(obsidian_turns)
        } + 1;
        if turns-turns_required as isize >= 0 {
            best = best.max(recurse(blueprint,
                                    (ore + turns_required * ore_prod - blueprint.geode.0, ore_prod,
                                     clay + turns_required * clay_prod, clay_prod,
                                     obsidian + turns_required * obsidian_prod - blueprint.geode.1, obsidian_prod,
                                     geode + turns_required * geode_prod, geode_prod + 1), turns - turns_required as isize));
        } else {
            best = best.max(geode + geode_prod * turns as usize);
        }
    }

    best
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Blueprint {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: (usize, usize),
    pub geode: (usize, usize),
}
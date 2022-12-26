use std::collections::{HashMap, HashSet};
use std::io::BufRead;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut elves = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                for ch in line.chars() {
                    if ch == '#' {
                        elves.insert((y,x));
                    }
                    x += 1;
                }
                x = 0;
                y += 1;
            }
        }
    }

    // Loops from 0-3, indicating which direction should be checked first
    let mut priority = 0;
    let mut round = 0;
    loop {
        // Bitset!
        // 1 = east, 2 = south, 4 = west, 8 = north
        // Stores a coordinate (y,x) and from which directions elves have proposed moving
        let mut moves = HashMap::new();
        for (elf_y ,elf_x) in &elves {
            let (elf_y, elf_x) = (*elf_y, *elf_x);
            let north = elves.contains(&(elf_y - 1, elf_x - 1)) ||  elves.contains(&(elf_y - 1, elf_x)) || elves.contains(&(elf_y - 1, elf_x + 1));
            let south = elves.contains(&(elf_y + 1, elf_x - 1)) ||  elves.contains(&(elf_y + 1, elf_x)) || elves.contains(&(elf_y + 1, elf_x + 1));
            let west = elves.contains(&(elf_y - 1, elf_x - 1)) ||  elves.contains(&(elf_y, elf_x - 1)) || elves.contains(&(elf_y + 1, elf_x - 1));
            let east = elves.contains(&(elf_y - 1, elf_x + 1)) ||  elves.contains(&(elf_y, elf_x + 1)) || elves.contains(&(elf_y + 1, elf_x + 1));
            let can_move = (north || south || west || east) && !(north && south && west && east);
            if !can_move {
                continue;
            }
            // Currently directions container whether there's at least one elf in the 3 neighbouring cells in that direction
            // Inverting that tells us whether we can go that way or not
            let mut options = [!north, !south, !west, !east];
            let mut possible_moves = [
                ((elf_y-1,elf_x), moves.get(&(elf_y-1,elf_x)).unwrap_or(&2) | 2),
                ((elf_y+1,elf_x), moves.get(&(elf_y+1,elf_x)).unwrap_or(&8) | 8),
                ((elf_y,elf_x-1), moves.get(&(elf_y,elf_x-1)).unwrap_or(&1) | 1),
                ((elf_y,elf_x+1), moves.get(&(elf_y,elf_x+1)).unwrap_or(&4) | 4),
            ];
            options.rotate_left(priority);
            possible_moves.rotate_left(priority);
            for (idx, opt) in options.iter().enumerate() {
                if *opt {
                    moves.insert(possible_moves[idx].0, possible_moves[idx].1);
                    break;
                }
            }
        }

        let mut moved = false;
        for ((y,x), dirset) in moves {
            if dirset | 1 == 1 {
                assert!(elves.remove(&(y, x+1)));
                elves.insert((y,x));
                moved = true;
            } else if dirset | 2 == 2 {
                assert!(elves.remove(&(y+1, x)));
                elves.insert((y,x));
                moved = true;
            } else if dirset | 4 == 4 {
                assert!(elves.remove(&(y, x-1)));
                elves.insert((y,x));
                moved = true;
            } else if dirset | 8 == 8 {
                assert!(elves.remove(&(y-1, x)));
                elves.insert((y,x));
                moved = true;
            }
        }
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for (elf_y, elf_x) in &elves {
            min_x = min_x.min(*elf_x);
            max_x = max_x.max(*elf_x);
            min_y = min_y.min(*elf_y);
            max_y = max_y.max(*elf_y);
        }

        if !moved {
            break;
        }
        round += 1;
        if round >= 10 {
            break;
        }
        priority = (priority + 1) % 4;
    }

    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for (elf_y, elf_x) in &elves {
        min_x = min_x.min(*elf_x);
        max_x = max_x.max(*elf_x);
        min_y = min_y.min(*elf_y);
        max_y = max_y.max(*elf_y);
    }

    let diff_x = min_x.abs_diff(max_x) + 1;
    let diff_y = min_y.abs_diff(max_y) + 1;

    println!("{}", diff_x*diff_y - elves.len());
}

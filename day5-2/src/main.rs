use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    // First, read the initial configuration of crates
    let mut initial_stacks = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.expect("Read error");
        if line == "" {
            break;
        }
        initial_stacks.push(line);
    }
    // Stacks have the format '[A] ', except the last stack has no space
    // That makes it 4 characters per stack, except the last one
    let num_stacks = (initial_stacks[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];
    for i in (0..(initial_stacks.len()-1)).rev() {
        let crate_iter = &mut initial_stacks[i][1..].chars().step_by(4);
        for stack_index in 0..num_stacks {
            let symbol = crate_iter.next().expect("Ran out of symbols");
            if symbol != ' ' {
                stacks[stack_index].push(symbol);
            }
        }
    }

    // Now that we have the initial setup, run the instructions
    while let Some(line) = lines.next() {
        let line = line.expect("Read error");
        if line == "-" {
            break;
        }

        // Lines are of the format
        // 'move 1 from 1 to 2'
        // By splitting at spaces we get ['move', '1', 'from', '1', 'to', '2']
        let parts: Vec<&str> = line.split(" ").collect();
        let repeats = usize::from_str(parts[1]).expect("Invalid number");
        // Subtract one, since the instructs use 1-indexing
        let source = usize::from_str(parts[3]).expect("Invalid number") - 1;
        let destination = usize::from_str(parts[5]).expect("Invalid number") - 1;

        let split_at = stacks[source].len()-repeats;
        let mut symbols = stacks[source].split_off(split_at);
        stacks[destination].append(&mut symbols);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!()
}
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

/*
    Since the input does not have a known length, we interpret a '-' dash character as the end of the input
 */
pub fn main() {
    let input = std::io::stdin().lock();
    let mut lines = input.lines();

    let mut valves_raw = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "-" => {
                break;
            }
            _ => {
                let name = &line[line.find(" ").unwrap() + 1..line.find(" ").unwrap() + 3];
                let rate = usize::from_str(&line[line.find("rate=").unwrap() + 5..line.find(";").unwrap()]).unwrap();
                let destinations: Vec<String> = line[line.find("valve").unwrap() + 6..]
                    .trim()
                    .split(", ")
                    .map(|elem| {
                        elem.to_string()
                    })
                    .collect();

                valves_raw.push((name.to_string(), rate, destinations));
            }
        }
    }

    let mut valves = vec![(0, vec![]); valves_raw.len()];
    let mut mapping = HashMap::new();
    let mut count = 0;
    for valve_flow in valves_raw.iter().filter(|valve| valve.1 > 0) {
        mapping.insert(valve_flow.0.to_string(), mapping.len());
        valves[count].0 = valve_flow.1;
        count += 1;
    }
    let valves_with_flow = count;
    for valve_flow in valves_raw.iter().filter(|valve| valve.1 == 0) {
        mapping.insert(valve_flow.0.to_string(), mapping.len());
        valves[count].0 = valve_flow.1;
        count += 1;
    }
    for valve in valves_raw {
        let idx = *mapping.get(&valve.0).unwrap();
        for adjacent_valve in &valve.2 {
            let idx2 = *mapping.get(adjacent_valve).unwrap();
            valves[idx].1.push(idx2);
        }
    }

    // [rounds_left][current_valve][open_valves_bitset] = best_known_flow
    let mut memory: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 1usize << valves_with_flow]; valves.len()]; 30];
    let bitset_max = 1usize << valves_with_flow;
    // For every round (starting with the _last_ round)
    // Only need 26 rounds here, since we always "waste" 4 training the elephant
    for round in 1..26 {
        // For every valve...
        for valve in 0..valves.len() {
            let this_bit = 1usize << valve;
            // For every combination of open/closed valves
            for open in 0..bitset_max {
                let mut best_known = 0;
                // 1. If this valve gives flow when opened
                // 2. If this valve is not already open
                // 3. If there are enough rounds left that it will produce any flow (1 round to open and 1 round to produce)
                if valves[valve].0 != 0 && this_bit & open != 0 && round >= 2
                {
                    // Then best known is:
                    // Best for previous round where we were standing on this + rate * rounds it can produce
                    // We use previous round since we must have been here the previous round before it
                    // would be able to produce this round (since the round in which we open it adds no flow)
                    best_known = best_known.max(memory[round-1][valve][open - this_bit] + valves[valve].0 * round);
                }
                // For each adjacent valve
                for adjacent_valve in &valves[valve].1 {
                    // Check if there is a better known flow by being there instead (moving instead of opening)
                    best_known = best_known.max(memory[round-1][*adjacent_valve][open]);
                }
                memory[round][valve][open] = best_known;
            }
        }
    }

    let mut best = 0;
    // Loop over all the possibilities, split between us and the elephant
    // This will check all the possible combinations of valves that we open and valves the elephant opens along with ours
    // This will first check "what if we open none and the elephant opens all"
    // Then it checks "what if we open some specific valve and the elephant doesn't open that one"
    // Keep repeating for "what if I open these and the elephant doesn't open the same ones"
    // Eventually we get to "what if I open half of then and the elephant the other half"
    // At this point, we've covered all possibilities, since whether whether we consider ourselves
    // the "elephant" or "us" in the above example is interchangeable -- We can just never open the same valve
    for open in 0..bitset_max/2 {
        // Our set is the first half of the valves
        let my_set = open;
        // Elephant set is the other half
        let elephant_set = bitset_max - open - 1;
        // Best is the highest possible combination of our rate + elephant rate
        // We check round 26 (but with 0-indexing, so 25)
        best = best.max(memory[25][*mapping.get("AA").unwrap()][my_set] + memory[25][*mapping.get("AA").unwrap()][elephant_set]);
    }
    println!("{best}");
}
// Solution to Day 16 puzzle
// https://adventofcode.com/2022/day/16
//
// Example usage:
//   cargo run --bin day16 data/day16/test_input.txt
//
// Since this puzzle takes a while, recommend running with the -r flag for release profile.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

extern crate regex;
use regex::Regex;


#[derive(Debug)]
struct Node {
    step: usize,
    location: String,
    prev_location: String,
    elephant_location: String,
    prev_elephant_location: String,
    valves_open: HashSet<String>,
    score: u32,
}

fn get_flow_from_valves(valve_data: &HashMap<String, u32>,
                        valves_open: &HashSet<String>) -> u32 {
    let mut flow: u32 = 0;
    for valve in valves_open.iter() {
        flow += valve_data.get(valve).unwrap();
    }
    flow
}

fn max_flow_from_valves(valve_data: &HashMap<String, u32>) -> u32 {
    let mut max_flow: u32 = 0;
    for (_, flow) in valve_data {
        max_flow += flow;
    }
    max_flow
}

fn search_puzzle(valve_flow_data: &HashMap<String, u32>,
    valve_transition_data: &HashMap<String, String>,
    max_steps: usize,
    use_elephant: bool) -> u32 {

    // Hack to add an open
    let mut valve_transition_mod = HashMap::new();
    for (name, trans) in valve_transition_data {
        let mut mod_trans = trans.clone();
        mod_trans.push_str(", OPEN");
        valve_transition_mod.insert(name, mod_trans);
    }

    // Get number of useful valves
    let mut num_useful_valves = 0;
    for (_, flow) in valve_flow_data {
        if flow > &0 {
            num_useful_valves += 1;
        }
    }
    let max_valve_flow = max_flow_from_valves(&valve_flow_data);

    // Initialize
    let mut max_score = 0;
    let init_state = Node{
        step: 0, 
        prev_location: "AA".to_string(),
        location: "AA".to_string(),
        prev_elephant_location: "AA".to_string(),
        elephant_location: "AA".to_string(),
        valves_open: HashSet::new(),
        score: 0
    };
    let mut state_stack = Vec::new();
    state_stack.push(init_state);

    while !state_stack.is_empty() {
        // Add to the stack
        let state = state_stack.pop().unwrap();
        
        // If this is max step, don't add more to the stack
        if state.step >= max_steps {
            continue;
        }

        // Update the max running score
        let added_flow = get_flow_from_valves(&valve_flow_data, &state.valves_open);
        let new_score = state.score + added_flow;
        if new_score >= max_score {
            // println!("Max score {} at step {}", max_score, state.step + 1);
            max_score = new_score;
        } else {
            // Optimism: If we opened all valves now and it still wouldnt be enough to hit max score,
            // we can safely ignore expanding this node further.
            let optimistic_score = state.score + max_valve_flow * ((max_steps - state.step) as u32);
            if optimistic_score <= max_score {
                continue;
            }
        }

        // If all valves are open
        let all_valves_open = state.valves_open.len() == num_useful_valves;
        if all_valves_open {
            state_stack.push(
                Node {
                    step: state.step + 1,
                    prev_location: state.prev_location.clone(),
                    location: state.location.clone(),
                    prev_elephant_location: state.prev_elephant_location.clone(),
                    elephant_location: state.elephant_location.clone(),
                    valves_open: state.valves_open.clone(),
                    score: new_score,
                }
            );
            continue;
        }

        // Add transitions, pruning cyclic ones by using the previous location
        let transition_str = valve_transition_mod.get(&state.location).unwrap();

        for transition in transition_str.split(", ") {                    
            let mut new_valves_open = state.valves_open.clone();

            let mut dest_self = transition.to_string();
            let open_self = dest_self == "OPEN";
            if open_self {
                dest_self = state.location.clone();
                let flow_zero = *valve_flow_data.get(&state.location).unwrap() == 0;
                let cur_valve_open = state.valves_open.contains(&state.location);
                if !cur_valve_open && !flow_zero {
                    new_valves_open.insert(state.location.clone());
                } else {
                    continue;
                }
            }

            let is_cyclic = (dest_self == state.prev_location) && !open_self;
            if is_cyclic {
                continue;
            }

            // If not using elephant, add node right away
            if !use_elephant {
                state_stack.push(
                    Node {
                        step: state.step + 1,
                        prev_location: state.location.clone(),
                        location: dest_self.clone(),
                        prev_elephant_location: state.prev_elephant_location.clone(),
                        elephant_location: state.elephant_location.clone(),
                        valves_open: new_valves_open.clone(),
                        score: new_score,
                    }
                );
                continue;
            }

            // If using elephant, go through its possible transitions
            let transition_str_ele = valve_transition_mod.get(&state.elephant_location).unwrap();
            for transition_ele in transition_str_ele.split(", ") {  
                let mut ele_valves_open = new_valves_open.clone();                  
                let mut dest_ele = transition_ele.to_string();
                let open_ele = dest_ele == "OPEN";
                if open_ele {
                    dest_ele = state.elephant_location.clone();
                    let flow_zero = *valve_flow_data.get(&state.elephant_location).unwrap() == 0;
                    let cur_valve_open = ele_valves_open.contains(&state.elephant_location);
                    if !cur_valve_open && !flow_zero {
                        ele_valves_open.insert(state.elephant_location.clone());
                    } else {
                        continue;
                    }
                }

                let is_cyclic_ele = (dest_ele == state.prev_elephant_location) && !open_ele;
                if is_cyclic_ele {
                    continue;
                }

                // Add node if we passed all tests
                state_stack.push(
                    Node {
                        step: state.step + 1,
                        prev_location: state.location.clone(),
                        location: dest_self.clone(),
                        prev_elephant_location: state.elephant_location.clone(),
                        elephant_location: dest_ele.clone(),
                        valves_open: ele_valves_open.clone(),
                        score: new_score,
                    }
                );
            }
        }
    }

    max_score
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day16/test_input.txt" };

    // Parse the data
    let data = fs::read_to_string(filename).unwrap();

    let expr = r"Valve ([A-Z][A-Z]) has flow rate=(\d*); tunnel[s]* lead[s]* to valve[s]* (.*)";
    let re = Regex::new(expr).unwrap();

    let mut valve_flow_data = HashMap::new();
    let mut valve_transition_data = HashMap::new();
    for line in data.lines() {
        let cap = re.captures(line).unwrap();
        let valve_name = (&cap[1]).to_string();
        let valve_flow = (&cap[2]).to_string().parse::<u32>().unwrap();
        let valve_transitions = (&cap[3]).to_string();

        valve_flow_data.insert(valve_name.clone(), valve_flow);
        valve_transition_data.insert(valve_name.clone(), valve_transitions);
    }
    
    // Part 1
    let max_steps: usize = 30;
    let max_score = search_puzzle(&valve_flow_data, &valve_transition_data, max_steps, false);
    println!("\nPart 1: Max possible score after {} steps = {}\n", max_steps, max_score);

    // Part 2
    let max_steps: usize = 26;
    let max_score = search_puzzle(&valve_flow_data, &valve_transition_data, max_steps, true);
    println!("\nPart 2: Max possible score after {} steps = {}\n", max_steps, max_score);
}

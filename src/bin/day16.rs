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
    valves_open: HashSet<String>,
    score: u32,
}

#[derive(Debug)]
struct NodeWithElephant {
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

// Part 1
fn part1(valve_flow_data: &HashMap<String, u32>,
         valve_transition_data: &HashMap<String, String>,
         max_steps: usize) -> u32 {

    // Initialize
    let mut max_score = 0;
    let init_state = Node{
        step: 0, 
        prev_location: "AA".to_string(),
        location: "AA".to_string(),
        valves_open: HashSet::new(),
        score: 0
    };
    let mut state_stack = Vec::new();
    state_stack.push(init_state);

    while !state_stack.is_empty() {
        // Add to the stack
        let state = state_stack.pop().unwrap();
        
        // If this is max step, don't add more to the stack
        if state.step == max_steps {
            continue;
        }

        // Update the max running score
        let new_score = state.score + get_flow_from_valves(&valve_flow_data, &state.valves_open);
        if new_score > max_score {
            // println!("Update max score to {}", max_score);
            max_score = new_score;
        }

        // Add transitions, pruning cyclic ones by using the previous location
        let transition_str = valve_transition_data.get(&state.location).unwrap();
        for transition in transition_str.split(", ") {
            if transition.to_string() == state.prev_location {
                continue;
            }
            
            let new_node = Node {
                step: state.step + 1,
                prev_location: state.location.clone(),
                location: transition.to_string(),
                valves_open: state.valves_open.clone(),
                score: new_score,
            };
            // println!("Added node: {:?}", new_node);
            state_stack.push(new_node);
        }

        // Add valve open
        let flow_zero = *valve_flow_data.get(&state.location).unwrap() == 0;
        let cur_valve_open = state.valves_open.contains(&state.location);
        if !cur_valve_open && !flow_zero {
            let mut new_valves_open = state.valves_open.clone();
            new_valves_open.insert(state.location.clone());
            let new_node = Node { 
                step: state.step + 1,
                prev_location: state.location.clone(),
                location: state.location.clone(),
                valves_open: new_valves_open,
                score: new_score,
            };
            //println!("Added node: {:?}", new_node);
            state_stack.push(new_node);
        }
    }

    max_score
}

// Part 2
fn part2(valve_flow_data: &HashMap<String, u32>,
    valve_transition_data: &HashMap<String, String>,
    max_steps: usize) -> u32 {

    // hack it
    let mut valve_transition_mod = HashMap::new();
    for (name, trans) in valve_transition_data {
        let mut mod_trans = trans.clone();
        mod_trans.push_str(", OPEN");
        valve_transition_mod.insert(name, mod_trans);
    }

    // Initialize
    let mut max_score = 0;
    let init_state = NodeWithElephant{
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
        if state.step == max_steps {
            continue;
        }

        // Update the max running score
        let new_score = state.score + get_flow_from_valves(&valve_flow_data, &state.valves_open);
        if new_score > max_score {
            println!("Update max score to {}", max_score);
            max_score = new_score;
        }

        // Add transitions, pruning cyclic ones by using the previous location
        let transition_str = valve_transition_mod.get(&state.location).unwrap();
        let transition_elephant_str = valve_transition_mod.get(&state.elephant_location).unwrap();

        for transition in transition_str.split(", ") {
                for transition_elephant in transition_elephant_str.split(", ") {
                    let mut new_valves_open = state.valves_open.clone();

                    let mut dest_self = transition.to_string();
                    let open_self = dest_self == "OPEN";
                    if open_self {
                        dest_self = state.prev_location.clone();
                        let flow_zero = *valve_flow_data.get(&state.location).unwrap() == 0;
                        let cur_valve_open = state.valves_open.contains(&state.location);
                        if !cur_valve_open && !flow_zero {
                            new_valves_open.insert(state.location.clone());
                        } else {
                            continue;
                        }
                    }

                    let mut dest_ele = transition_elephant.to_string();
                    let open_ele = dest_ele == "OPEN";
                    if open_ele {
                        dest_ele = state.prev_elephant_location.clone();
                        let flow_zero = *valve_flow_data.get(&state.elephant_location).unwrap() == 0;
                        let cur_valve_open = new_valves_open.contains(&state.elephant_location);
                        if !cur_valve_open && !flow_zero {
                            new_valves_open.insert(state.elephant_location.clone());
                        } else {
                            continue;
                        }
                    }

                    let is_cyclic = (dest_self == state.prev_location) &&
                                    (dest_ele == state.prev_elephant_location);
                    if is_cyclic {
                        continue;
                    }

                    // println!("Feasible transition, self {} to {} [{}], elephant {} to {} [{}]",
                    //     state.location.clone(), dest_self, open_self,
                    //     state.elephant_location.clone(), dest_ele, open_ele);

                    // Add node if we passed all tests
                    let new_node = NodeWithElephant {
                        step: state.step + 1,
                        prev_location: state.location.clone(),
                        location: dest_self.clone(),
                        prev_elephant_location: state.elephant_location.clone(),
                        elephant_location: dest_ele.clone(),
                        valves_open: new_valves_open.clone(),
                        score: new_score,
                    };
                    // println!("Added node: {:?}", new_node);
                    state_stack.push(new_node);
                }
        }

        // // Add valve open
        // let flow_zero = *valve_flow_data.get(&state.location).unwrap() == 0;
        // let cur_valve_open = state.valves_open.contains(&state.location);
        // if !cur_valve_open && !flow_zero {
        //     let mut new_valves_open = state.valves_open.clone();
        //     new_valves_open.insert(state.location.clone());
        //     let new_node = NodeWithElephant { 
        //         step: state.step + 1,
        //         prev_location: state.location.clone(),
        //         location: state.location.clone(),
        //         prev_elephant_location: state.elephant_location.clone(),
        //         elephant_location: state.elephant_location.clone(),
        //         valves_open: new_valves_open,
        //         score: new_score,
        //     };
        //     //println!("Added node: {:?}", new_node);
        //     state_stack.push(new_node);
        // }
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
    // let max_steps: usize = 30;
    // let max_score = part1(&valve_flow_data, &valve_transition_data, max_steps);
    // println!("\nPart 1: Max possible score after {} steps = {}\n", max_steps, max_score);

    // Part 2
    let max_steps: usize = 26;
    let max_score = part2(&valve_flow_data, &valve_transition_data, max_steps);
    println!("\nPart 2: Max possible score after {} steps = {}\n", max_steps, max_score);
}

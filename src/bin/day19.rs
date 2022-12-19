// Solution to Day 19 puzzle
// https://adventofcode.com/2022/day/19
//
// Example usage:
//   cargo run --bin day19 data/day19/test_input.txt

use std::env;
use std::fs;

extern crate regex;
use regex::Regex;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Blueprint {
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    step: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    n_ore_robots: u32,
    n_clay_robots: u32,
    n_obsidian_robots: u32,
    n_geode_robots: u32,
    ore_robot_building: u32,
    clay_robot_building: u32,
    obsidian_robot_building: u32,
    geode_robot_building: u32,
}


fn parse_blueprints(filename: &str) -> Vec<Blueprint> {
    let expr = "Each ore robot costs ([0-9]*) ore. \
                Each clay robot costs ([0-9]*) ore. \
                Each obsidian robot costs ([0-9]*) ore and ([0-9]*) clay. \
                Each geode robot costs ([0-9]*) ore and ([0-9]*) obsidian.";
    let re = Regex::new(expr).unwrap();
    
    let data = fs::read_to_string(filename).unwrap();
    let mut blueprints = Vec::<Blueprint>::new();
    for line in data.lines() {
        let cap = re.captures(line).unwrap();
        blueprints.push(
            Blueprint {
                ore_robot_ore_cost: (&cap[1]).to_string().parse::<u32>().unwrap(),
                clay_robot_ore_cost: (&cap[2]).to_string().parse::<u32>().unwrap(),
                obsidian_robot_ore_cost: (&cap[3]).to_string().parse::<u32>().unwrap(),
                obsidian_robot_clay_cost: (&cap[4]).to_string().parse::<u32>().unwrap(),
                geode_robot_ore_cost: (&cap[5]).to_string().parse::<u32>().unwrap(),
                geode_robot_obsidian_cost: (&cap[6]).to_string().parse::<u32>().unwrap()
            }
        );
    }
    blueprints
}

fn get_max_geodes(blueprint: &Blueprint, n_steps: u32) -> u32 {
    let mut max_geodes = 0;
    let mut max_geodes_per_step = vec![0; (n_steps + 1) as usize];

    // Initialize
    let init_state = State {
        step: 0,
        ore: 0, clay: 0, obsidian: 0, geode: 0,
        n_ore_robots: 1, n_clay_robots: 0, n_obsidian_robots: 0, n_geode_robots: 0,
        ore_robot_building: 0, clay_robot_building: 0, obsidian_robot_building: 0, geode_robot_building: 0
    };
    let mut state_stack = vec![init_state];

    // Simulate
    while !state_stack.is_empty() {
        let mut cur_state = state_stack.pop().unwrap();

        // If this is max step, don't add more to the stack
        if cur_state.step >= n_steps {
            continue;
        }
        // If we know we can have more geodes at this step from another path,
        // skip expanding this node any further.
        if max_geodes_per_step[cur_state.step as usize] > cur_state.geode {
            continue;
        }

        // Simulate forward
        cur_state.step += 1;
        // Resources
        cur_state.ore += cur_state.n_ore_robots;
        cur_state.clay += cur_state.n_clay_robots;
        cur_state.obsidian += cur_state.n_obsidian_robots;
        cur_state.geode += cur_state.n_geode_robots;
        // Building
        if cur_state.ore_robot_building > 0 {
            cur_state.ore_robot_building = 0;
            cur_state.n_ore_robots += 1;
        }
        if cur_state.clay_robot_building > 0 {
            cur_state.clay_robot_building = 0;
            cur_state.n_clay_robots += 1;
        }
        if cur_state.obsidian_robot_building > 0 {
            cur_state.obsidian_robot_building = 0;
            cur_state.n_obsidian_robots += 1;
        }
        if cur_state.geode_robot_building > 0 {
            cur_state.geode_robot_building = 0;
            cur_state.n_geode_robots += 1;
        }

        // Check max geodes
        if cur_state.geode > max_geodes {
            max_geodes = cur_state.geode;
            println!("  Max geodes so far: {}", max_geodes);
        }
        if cur_state.geode > max_geodes_per_step[cur_state.step as usize] {
            max_geodes_per_step[cur_state.step as usize] = cur_state.geode;
        }

        // Add possible transitions
        // Geode robot
        if cur_state.ore >= blueprint.geode_robot_ore_cost &&
           cur_state.obsidian >= blueprint.geode_robot_obsidian_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.geode_robot_ore_cost;
            new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
            new_state.geode_robot_building += 1;
            state_stack.push(new_state);
        }
        // Obsidian robot
        if cur_state.ore >= blueprint.obsidian_robot_ore_cost &&
           cur_state.clay >= blueprint.obsidian_robot_clay_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
            new_state.obsidian_robot_building += 1;
            state_stack.push(new_state);
        }
        // Clay robot
        if cur_state.ore >= blueprint.clay_robot_ore_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.clay_robot_ore_cost;
            new_state.clay_robot_building += 1;
            state_stack.push(new_state);
        }
        // Ore robot
        if cur_state.ore >= blueprint.ore_robot_ore_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.ore_robot_ore_cost;
            new_state.ore_robot_building = 1;
            state_stack.push(new_state);
        }
        // Do nothing
        state_stack.push(cur_state.clone());
    }

    max_geodes
}

fn get_quality_level(blueprints: &Vec<Blueprint>, n_steps: u32) -> u32 {

    let mut quality_level = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {}:", idx + 1);
        let n_geodes = get_max_geodes(&blueprint, n_steps);
        quality_level += ((idx + 1) as u32) * n_geodes;
        println!("Produced {} geodes\n", n_geodes);
    }
    quality_level
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day19/test_input.txt" };
    
    let blueprints = parse_blueprints(&filename);

    // Part 1
    let quality_level = get_quality_level(&blueprints, 24);
    println!("Part 1: Quality level = {}", quality_level);
}

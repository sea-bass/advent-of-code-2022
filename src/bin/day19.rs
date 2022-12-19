// Solution to Day 19 puzzle
// https://adventofcode.com/2022/day/19
//
// Example usage:
//   cargo run --bin day19 data/day19/test_input.txt

use std::cmp::{min, max};
use std::env;
use std::fs;

extern crate rayon;
use rayon::prelude::*;
extern crate regex;
use regex::Regex;


// Structs to describe the puzzle
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Blueprint {
    id: u32,
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


// Input file parsing helper
fn parse_blueprints(filename: &str) -> Vec<Blueprint> {
    let expr = "Blueprint ([0-9]*): \
                Each ore robot costs ([0-9]*) ore. \
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
                id: (&cap[1]).to_string().parse::<u32>().unwrap(),
                ore_robot_ore_cost: (&cap[2]).to_string().parse::<u32>().unwrap(),
                clay_robot_ore_cost: (&cap[3]).to_string().parse::<u32>().unwrap(),
                obsidian_robot_ore_cost: (&cap[4]).to_string().parse::<u32>().unwrap(),
                obsidian_robot_clay_cost: (&cap[5]).to_string().parse::<u32>().unwrap(),
                geode_robot_ore_cost: (&cap[6]).to_string().parse::<u32>().unwrap(),
                geode_robot_obsidian_cost: (&cap[7]).to_string().parse::<u32>().unwrap()
            }
        );
    }
    blueprints
}

// Main simulation function
fn get_max_geodes(blueprint: &Blueprint, n_steps: u32) -> u32 {
    // Initialize
    let init_state = State {
        step: 0,
        ore: 0, clay: 0, obsidian: 0, geode: 0,
        n_ore_robots: 1, n_clay_robots: 0, n_obsidian_robots: 0, n_geode_robots: 0,
        ore_robot_building: 0, clay_robot_building: 0, obsidian_robot_building: 0, geode_robot_building: 0
    };
    let mut state_stack = vec![init_state];

    let mut max_geodes = 0;
    let mut max_geodes_per_step = vec![0; (n_steps + 1) as usize];

    let max_necessary_ore_production = max(
        blueprint.ore_robot_ore_cost, 
        max(
            blueprint.clay_robot_ore_cost,
            max (blueprint.obsidian_robot_ore_cost,
                 blueprint.geode_robot_ore_cost)
        )
    );

    // Simulate
    while !state_stack.is_empty() {
        let mut cur_state = state_stack.pop().unwrap();

        // If this is max step, don't add more to the stack
        if cur_state.step >= n_steps {
            continue;
        }
        // If we know we can have more geodes at this step from another path,
        // skip expanding this node any further.
        // This fudge factor of 2 works, but I'm not sure why this did and 1 didn't...
        if max_geodes_per_step[cur_state.step as usize] > cur_state.geode + 2 {
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
            // println!("  [Blueprint {}] Max geodes so far: {}", blueprint.id, max_geodes);
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
            continue;
        }
        // Obsidian robot
        if cur_state.n_obsidian_robots < blueprint.geode_robot_obsidian_cost &&
           cur_state.ore >= blueprint.obsidian_robot_ore_cost &&
           cur_state.clay >= blueprint.obsidian_robot_clay_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
            new_state.obsidian_robot_building += 1;
            state_stack.push(new_state);
        }
        // Clay robot
        if cur_state.n_clay_robots < blueprint.obsidian_robot_clay_cost && 
           cur_state.ore >= blueprint.clay_robot_ore_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.clay_robot_ore_cost;
            new_state.clay_robot_building += 1;
            state_stack.push(new_state);
        }
        // Ore robot
        if cur_state.n_ore_robots < max_necessary_ore_production &&
           cur_state.ore >= blueprint.ore_robot_ore_cost {
            let mut new_state = cur_state.clone();
            new_state.ore -= blueprint.ore_robot_ore_cost;
            new_state.ore_robot_building = 1;
            state_stack.push(new_state);
        }
        // Do nothing
        state_stack.push(cur_state.clone());
    }

    println!("[Blueprint {}] Produced {} geodes\n", blueprint.id, max_geodes);
    max_geodes
}


// Part 1 entry point
fn get_quality_level(blueprints: &Vec<Blueprint>, n_steps: u32) -> u32 {
    blueprints.par_iter()
              .map(|b| b.id * get_max_geodes(&b, n_steps))
              .sum()
}

// Part 2 entry point
fn get_geode_product(blueprints: &Vec<Blueprint>, n_steps: u32, n_blueprints: usize) -> u32 {
    let n = min(n_blueprints, blueprints.len());
    blueprints[0..n].par_iter()
                    .map(|b| get_max_geodes(&b, n_steps))
                    .product()
}


fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day19/test_input.txt" };
    
    let blueprints = parse_blueprints(&filename);

    // Part 1
    let quality_level = get_quality_level(&blueprints, 24);
    println!("\nPart 1: Quality level = {}\n", quality_level);

    // Part 2
    let geode_product = get_geode_product(&blueprints, 32, 3);
    println!("\nPart 2: Geode multiple = {}\n", geode_product);
}

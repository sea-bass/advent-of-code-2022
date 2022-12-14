// Solution to Day 14 puzzle
// https://adventofcode.com/2022/day/14
//
// Example usage:
//   cargo run --bin day14 data/day14/test_input.txt

use std::cmp::{min, max};
use std::env;
use std::fs;

extern crate ndarray;
use ndarray::{Array2, s};

const GRID_OFFSET: usize = 1000;
const GRID_SIZE: usize = 2000;
const SAND_VAL: u32 = 1;
const ROCK_VAL: u32 = 2;
const SAND_ORIGIN_X: usize = 500;
const SAND_ORIGIN_Y: usize = 0;

fn sand_sim(filename: &str, add_floor: bool) -> u32 {
    // Initialize the grid
    let mut grid = Array2::<u32>::zeros((GRID_SIZE, GRID_SIZE));

    // Read the file to fill in rocks in the grid
    let mut floor_height = 0;
    let mut min_x: usize =  1000;
    let mut max_x: usize = 0;
    let data = fs::read_to_string(filename).unwrap();
    for line in data.lines() {
        let pts = line.split(" -> ")
                    .map(|x| x.split(",")
                            .map(|y| y.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                        )
                    .collect::<Vec<Vec<usize>>>();

        for i in 1..pts.len() {
            let start = &pts[i-1];
            let end = &pts[i];
            let x_start = GRID_OFFSET + min(start[0], end[0]);
            let y_start = min(start[1], end[1]);
            let x_end = GRID_OFFSET + max(start[0], end[0]);
            let y_end = max(start[1], end[1]);
            for elem in grid.slice_mut(s![y_start..=y_end, x_start..=x_end]) {
                *elem = ROCK_VAL;
            }

            min_x = min(min_x, x_start);
            max_x = max(max_x, x_end);
            floor_height = max(floor_height, y_end + 2);
        }
    }
    
    // Add floor, if enabled
    if add_floor {
        println!("Added floor at height {}", floor_height);
        for elem in grid.slice_mut(s![floor_height, ..]) {
            *elem = ROCK_VAL;
        }
    }
    
    // Simulate sand
    let mut sim_done = false;
    let mut num_grains_at_rest = 0;
    while !sim_done {
        let mut grain_done = false;
        let mut grain_pos = [SAND_ORIGIN_Y, GRID_OFFSET + SAND_ORIGIN_X];
        while !grain_done {
            let down_pos = [grain_pos[0] + 1, grain_pos[1]];            
            match grid.get(down_pos) {
                Some(val) => {
                    if *val == 0 {
                        grain_pos = down_pos;
                        continue;
                    }
                },
                None => {
                    println!("Grain overflowed on bottom");
                    sim_done = true;
                    break;
                }
            }

            let left_pos = [grain_pos[0] + 1, grain_pos[1] - 1];
            match grid.get(left_pos) {
                Some(val) => {
                    if *val == 0 {
                        grain_pos = left_pos;
                        continue;
                    }
                },
                None => {
                    println!("Grain overflowed on left");
                    sim_done = true;
                    break;
                }
            }

            let right_pos = [grain_pos[0] + 1, grain_pos[1] + 1];
            match grid.get(right_pos) {
                Some(val) => {
                    if *val == 0 {
                        grain_pos = right_pos;
                        continue;
                    }
                },
                None => {
                    println!("Grain overflows on right");
                    sim_done = true;
                    break;
                }
            }

            
            grid[grain_pos] = SAND_VAL;
            grain_done = true;
            num_grains_at_rest += 1;
            // println!("Grain {} done at {}, {}", num_grains_at_rest, down_pos[0], down_pos[1]);

            // Check if we overfilled on top
            if grain_pos[0] == SAND_ORIGIN_Y && grain_pos[1] == GRID_OFFSET + SAND_ORIGIN_X {
                println!("Overfilled with sand!");
                sim_done = true;
            }
        }
    }

    println!("\n{:?}", grid.slice(s![0..15, 493..505]));
    num_grains_at_rest
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day14/test_input.txt" };

    // Part 1
    let num_grains_1 = sand_sim(filename, false);

    // Part 2
    let num_grains_2 = sand_sim(filename, true);

    println!("\nPart 1: Ended with {} grains at rest\n", num_grains_1);
    println!("\nPart 2: Ended with {} grains at rest\n", num_grains_2);
}

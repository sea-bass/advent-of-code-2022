// Solution to Day 17 puzzle
// https://adventofcode.com/2022/day/17
//
// Example usage:
//   cargo run --bin day17 data/day17/test_input.txt

use std::cmp::max;
use std::env;
use std::fs;

extern crate ndarray;
use ndarray::{Array2, s};


const GRID_WIDTH: usize = 7;
const GRID_HEIGHT: usize = 1000000;  // Make this really big

type RockPoints = Vec<[i32; 2]>;

fn init_rock_pts() -> Vec<RockPoints> {
    let mut rock_vec = Vec::new();
    // Rock 1:
    // ####
    rock_vec.push(
        vec![[0,0], [0,1], [0,2], [0,3]]
    );
    // Rock 2:
    // .#.
    // ###
    // .#.
    rock_vec.push(
        vec![[0,1], [-1,0], [-1,1], [-1,2], [-2,1]]
    );
    // Rock 3:
    // ..#
    // ..#
    // ###
    rock_vec.push(
        vec![[0,0], [0,1], [0,2], [-1,2], [-2,2]]
    );
    // Rock 4:
    // #
    // #
    // #
    // #
    rock_vec.push(
        vec![[0,0], [-1,0], [-2,0], [-3,0]]
    );
    // Rock 5:
    // ##
    // ##
    rock_vec.push(
        vec![[0,0], [0,1], [-1,0], [-1,1]]
    );
    // Return
    rock_vec
}

fn get_rock_pts(init_pts: &RockPoints, origin: [i32; 2]) -> RockPoints {
    let mut rock_pts = Vec::new();

    let x = origin[0];
    let y = origin[1];
    for elem in init_pts.iter() {
        rock_pts.push([elem[0] + x, elem[1] + y]);
    }
    rock_pts
}

fn check_contact(grid: &Array2::<u32>, pts: &RockPoints) -> bool {
    let shape = grid.shape();
    let nr = shape[0] as i32;
    let nc = shape[1] as i32;
    for pt in pts.iter() {
        if pt[0] < 0 || pt[0] >= nr || pt[1] < 0 || pt[1] >= nc ||
           grid[[pt[0] as usize, pt[1] as usize]] > 0 {
            return true;
        }
    }
    false
}

fn get_gust_dir(gusts: &String, step: usize) -> i32 {
    let true_idx = step % gusts.len();
    let gust = gusts.chars().nth(true_idx).unwrap();
    match gust {
        '>' => 1,
        '<' => -1,
        _ => {
            println!("WARNING: INVALID GUST {}", gust);
            0
        }
    }
}

fn rock_sim(grid: &mut Array2::<u32>, gusts: &String, num_rocks: u32) -> i32 {

    let rock_pts = init_rock_pts();
    let num_rock_types = rock_pts.len() as u32;
    println!("{:?}", rock_pts);

    let mut step = 0;
    let mut tower_height: i32 = 0;
    for r in 0..num_rocks {
        let mut row: i32 = (GRID_HEIGHT as i32) - 1 - tower_height - 3;
        let mut col: i32 = 2;

        let rock_idx = (r % num_rock_types) as usize;
        let mut rock_stopped = false;
        println!("\nSimulating rock {}, type {}", r, rock_idx + 1);

        // Simulate a single rock falling to completion
        let mut pts = get_rock_pts(&rock_pts[rock_idx], [row, col]);
        while !rock_stopped {
            let mut new_pts = pts.clone();

            // Try move according to gust.
            // If in collision, nothing happens
            let dir = get_gust_dir(&gusts, step);
            for pt in new_pts.iter_mut() {
                if dir == 1 {
                    pt[1] += 1;
                } else {
                    pt[1] -= 1;
                }
            }
            if check_contact(&grid, &new_pts) {
                println!("No gust");
                new_pts = pts.clone();
            } else {
                println!("Gust pushes rock in {}", dir);
                pts = new_pts;
            }

            // Try move down.
            // If in collision, the rock stops
            let mut new_pts = pts.clone();
            for pt in new_pts.iter_mut() {
                pt[0] += 1;
            }
            if !check_contact(&grid, &new_pts) {
                pts = new_pts;
                println!("Collision free, stepping");
            } else {
                // Update the tower height if stopped
                println!("Rock stopped at {}, {}", row, col);
                for pt in pts.iter() {
                    grid[[pt[0] as usize, pt[1] as usize]] = (rock_idx as u32) + 1;
                    let pt_height = GRID_HEIGHT as i32 - pt[0] as i32;
                    tower_height = max(tower_height, pt_height);
                }
                println!("New tower height: {}", tower_height);
                rock_stopped = true;
            }

            // Update step number
            step += 1;
        }
    }

    tower_height
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day17/test_input.txt" };
    let gusts = fs::read_to_string(filename).unwrap();

    // Part 1
    let num_steps = 2022;
    let display_height = 100;
    let mut grid = Array2::<u32>::zeros((GRID_HEIGHT, GRID_WIDTH));
    let tower_height = rock_sim(&mut grid, &gusts, num_steps);
    println!("\n{:?}", grid.slice(s![GRID_HEIGHT-display_height..GRID_HEIGHT, ..]));
    println!("\nPart 1: Tower height {}\n", tower_height);
}

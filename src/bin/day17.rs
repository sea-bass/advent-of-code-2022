// Solution to Day 17 puzzle
// https://adventofcode.com/2022/day/17
//
// Example usage:
//   cargo run --bin day17 data/day17/test_input.txt
//
// Since this puzzle takes a while, recommend running with the -r flag for release profile.

use std::cmp::max;
use std::env;
use std::fs;

extern crate ndarray;
use ndarray::{Array2, s};


const GRID_WIDTH: usize = 7;
const GRID_HEIGHT: usize = 100000;  // No need to make this so big since we'll be doing a rolling buffer
const BUFFER_WINDOW: usize = 50000;
const BUFFER_THRESH: i64 = 10;

type RockPoints = Vec<[i64; 2]>;

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

fn get_rock_pts(init_pts: &RockPoints, origin: [i64; 2]) -> RockPoints {
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
    let nr = shape[0] as i64;
    let nc = shape[1] as i64;
    for pt in pts.iter() {
        if pt[0] < 0 || pt[0] >= nr || pt[1] < 0 || pt[1] >= nc ||
           grid[[pt[0] as usize, pt[1] as usize]] > 0 {
            return true;
        }
    }
    false
}

fn get_gust_dirs(gusts: &str) -> Vec<i64> {
    let mut gust_vec = Vec::new();
    for ch in gusts.chars() {
        gust_vec.push(
            match ch {
                '>' => 1,
                '<' => -1,
                _ => {
                    println!("WARNING: INVALID GUST {}", ch);
                    0
                },
            }
        );
    }
    gust_vec
}

fn rock_sim(grid: &mut Array2::<u32>, gusts: &String, num_rocks: i64) -> i64 {

    // Initialize
    let mut new_grid;
    let rock_pts = init_rock_pts();
    let num_rock_types = rock_pts.len() as i64;
    let gust_vec = get_gust_dirs(&gusts[..]);
    let num_gusts = gust_vec.len() as i64;
    let mut step: i64 = 0;
    let mut tower_height: i64 = 0;
    let mut buffer_height: i64 = 0;

    let mut pattern = Vec::<i64>::new();
    for p in 0..num_rock_types {
        pattern.push(-1);
    }

    // Loop through all the steps
    for r in 0..num_rocks {
        let row: i64 = (GRID_HEIGHT as i64) - 1 - (tower_height - buffer_height) - 3;
        let col: i64 = 2;
        let rock_idx = (r % num_rock_types) as usize;
        let mut rock_stopped = false;
        
        if r % 100000000 == 0 {
            println!("Simulating rock {} / {} [{}%]",
                r, num_rocks, (r as f64 / num_rocks as f64) * 100.0);
        }

        // Simulate a single rock falling to completion
        let mut pts = get_rock_pts(&rock_pts[rock_idx], [row, col]);
        while !rock_stopped {
            let mut new_pts = pts.clone();

            // Try move according to gust.
            // If in collision, nothing happens
            // let dir = get_gust_dir(&gusts[..], step);
            let dir = gust_vec[(step % num_gusts) as usize];
            for pt in new_pts.iter_mut() {
                if dir == 1 {
                    pt[1] += 1;
                } else {
                    pt[1] -= 1;
                }
            }
            if check_contact(&grid, &new_pts) {
                // println!("No gust");
                // new_pts = pts.clone();
            } else {
                // println!("Gust pushes rock in {}", dir);
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
                // println!("Collision free, stepping");
            } else {
                // Update the tower height if stopped
                // println!("Rock stopped at {}, {}", row, col);
                for pt in pts.iter() {
                    grid[[pt[0] as usize, pt[1] as usize]] = (rock_idx as u32) + 1;
                    let pt_height = GRID_HEIGHT as i64 - pt[0] as i64 + buffer_height;
                    tower_height = max(tower_height, pt_height);
                }
                // println!("New tower height: {}", tower_height);
                rock_stopped = true;
            }

            // Update step number
            step += 1;
        }

        // Update the rolling buffer if getting close to overflow
        let array_height = tower_height - buffer_height;
        if array_height > GRID_HEIGHT as i64 - BUFFER_THRESH {
            buffer_height += BUFFER_WINDOW as i64;
            new_grid = Array2::<u32>::zeros((GRID_HEIGHT, GRID_WIDTH));
            grid.slice(s![0..GRID_HEIGHT-BUFFER_WINDOW, ..])
                .assign_to(new_grid.slice_mut(s![BUFFER_WINDOW.., ..]));
            *grid = new_grid;
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
    let num_steps: i64 = 2022;
    let mut grid = Array2::<u32>::zeros((GRID_HEIGHT, GRID_WIDTH));
    let tower_height = rock_sim(&mut grid, &gusts, num_steps);
    // println!("\n{:?}", grid.slice(s![GRID_HEIGHT-100..GRID_HEIGHT, ..]));
    println!("\nPart 1: Tower height {}\n", tower_height);

    // Part 2
    let num_steps: i64 = 1000000000000;
    let mut grid = Array2::<u32>::zeros((GRID_HEIGHT, GRID_WIDTH));
    let tower_height = rock_sim(&mut grid, &gusts, num_steps);
    println!("\nPart 2: Tower height {}\n", tower_height);
}

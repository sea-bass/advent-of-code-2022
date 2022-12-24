// Solution to Day 24 puzzle
// https://adventofcode.com/2022/day/24
//
// Example usage:
//   cargo run -r --bin day24 data/day24/test_input.txt

use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

extern crate ndarray;
use ndarray::Array2;


// Handy types
type OccupancyGrid = Array2::<i32>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct BlizzardState {
    positions: Vec<(i32, i32)>,
    directions: Vec<String>,
}

// Parse the data
fn parse_data(filename: &str) -> (OccupancyGrid, BlizzardState) {
    let data = fs::read_to_string(filename).unwrap();
    let lines = data.lines().collect::<Vec<&str>>();

    // Initialize data structures
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut grid = OccupancyGrid::zeros((n_rows, n_cols));
    let mut blizzard_state = BlizzardState {
        positions: Vec::new(), directions: Vec::new()
    };

    // Loop through the characters in the data and fill the grid and blizzard sttae
    for (row, line) in lines.iter().enumerate() {
        for (col, elem) in line.chars().enumerate() {
            // Fill permanent walls
            if elem == '#' {
                grid[[row, col]] = 1;
            }
            // Fill blizzard state
            else if elem != '.' {
                blizzard_state.positions.push((row as i32, col as i32));
                blizzard_state.directions.push(elem.to_string());
            }
        }
    }

    return (grid, blizzard_state);
}

// Prints the state of the map
fn display_state(grid: &OccupancyGrid, blizzard_state: &BlizzardState) {
    let shape = grid.shape();
    let n_rows = shape[0];
    let n_cols = shape[1];

    let mut display_str = String::new();
    for r in 0..n_rows {
        for c in 0..n_cols {
            let display_char = match grid[[r, c]] {
                1 => "#".to_string(),
                _ => {
                    let mut ch = ".".to_string();
                    if let Some(idx) = blizzard_state.positions.iter()
                                       .position(|&p| p == (r as i32, c as i32)) {
                        ch = blizzard_state.directions[idx].to_string();
                    }
                    ch
                }
            };
            display_str.push_str(&display_char.to_owned());
        }
        display_str.push_str("\n");
    }
    println!("{}", display_str);
}

// Gets an occupancy grid given a blizzard state
fn get_grid_at_blizzard_state(grid: &OccupancyGrid, blizzard_state: &BlizzardState) -> OccupancyGrid {
    let mut new_grid = grid.clone();
    for pos in blizzard_state.positions.iter() {
        new_grid[[pos.0 as usize, pos.1 as usize]] = 1;
    }
    new_grid
}

// Steps the blizzard state
fn step_blizzard_state(grid: &OccupancyGrid, blizzard_state: &BlizzardState) -> BlizzardState {
    let shape = grid.shape();
    let n_rows = shape[0] as i32;
    let n_cols = shape[1] as i32;

    let mut new_blizzard_state = BlizzardState {
        positions: Vec::new(),
        directions: blizzard_state.directions.clone()
    };

    for (idx, pos) in blizzard_state.positions.iter().enumerate() {
        let dir = blizzard_state.directions[idx].chars().nth(0).unwrap();
        // Step
        let mut new_pos = match dir {
            '>' => (pos.0, pos.1 + 1),
            '<' => (pos.0, pos.1 - 1),
            '^' => (pos.0 - 1, pos.1),
            'v' => (pos.0 + 1, pos.1),
             _  => {
                println!("Warning! Invalid blizzard direction.");
                pos.clone()
             }
        };

        // Wrap around
        if new_pos.0 == 0 {
            new_pos = (n_rows - 2, new_pos.1);
        } else if new_pos.0 == n_rows - 1 {
            new_pos = (1, new_pos.1);
        }
        if new_pos.1 == 0 {
            new_pos = (new_pos.0, n_cols - 2);
        } else if new_pos.1 == n_cols - 1 {
            new_pos = (new_pos.0, 1);
        }

        new_blizzard_state.positions.push(new_pos);
    }

    new_blizzard_state
}

// Search function
fn search_blizzard(grid: &OccupancyGrid,
                   init_blizzard_state: &BlizzardState,
                   forward_dir: bool) -> (i32, BlizzardState) {
    let shape = grid.shape();
    let n_rows = shape[0] as i32;
    let n_cols = shape[1] as i32;

    // Assumes initial and goal positions are always the top left and bottom right of map,
    // unless the direction is flipped
    let init_pos: (i32, i32) = match forward_dir {
        true => (0, 1),
        false => (n_rows - 1, n_cols - 2)
    };
    let goal_pos: (i32, i32) = match forward_dir {
        true => (n_rows - 1, n_cols - 2),
        false => (0, 1)
    };

    // Initialize search
    // Search happens as BFS over (step_number, position) tuples.
    // When the step number increases, we can step the blizzard state and update the occupancy map.
    let mut state_vec = VecDeque::new();
    state_vec.push_back((0, init_pos));
    let mut state_set = HashSet::new();
    state_set.insert((0, init_pos));

    // Initialize a vector of blizzard states and grids
    let cur_blizzard_state = step_blizzard_state(&grid, &init_blizzard_state);
    let mut blizzard_state_vec = Vec::new();
    blizzard_state_vec.push(cur_blizzard_state.clone());
    let mut grid_vec = Vec::new();
    grid_vec.push(get_grid_at_blizzard_state(&grid, &cur_blizzard_state));

    // Do the search
    while !state_vec.is_empty() {
        // Get the next state and check if it's the goal
        let state = state_vec.pop_front().unwrap();
        let cur_step = state.0;
        let cur_pos = state.1;
        if cur_pos == goal_pos {
            return (cur_step, blizzard_state_vec[cur_step as usize - 1].clone());
        }

        // Get the grid at the current state, creating it if not
        if cur_step > grid_vec.len() as i32 - 1 {
            let prev_blizzard_state = &blizzard_state_vec[(cur_step - 1) as usize];
            let new_blizzard_state = step_blizzard_state(&grid, &prev_blizzard_state);
            let new_grid = get_grid_at_blizzard_state(&grid, &new_blizzard_state);
            grid_vec.push(new_grid);
            blizzard_state_vec.push(new_blizzard_state);
        }
        let cur_grid = &grid_vec[cur_step as usize];
     
        // Expand nodes in all possible directions
        let right_pos = (cur_pos.0, cur_pos.1 + 1);
        if cur_grid[[right_pos.0 as usize, right_pos.1 as usize]] == 0 {
            let right_state = (cur_step + 1, right_pos);
            if !state_set.contains(&right_state) {
                state_vec.push_back(right_state);
                state_set.insert(right_state);
            }
        }
        let down_pos = (cur_pos.0 + 1, cur_pos.1);
        if down_pos.0 < n_rows && cur_grid[[down_pos.0 as usize, down_pos.1 as usize]] == 0 {
            let down_state = (cur_step + 1, down_pos);
            if !state_set.contains(&down_state) {
                state_vec.push_back(down_state);
                state_set.insert(down_state);
            }
        }
        let left_pos = (cur_pos.0, cur_pos.1 - 1);
        if cur_grid[[left_pos.0 as usize, left_pos.1 as usize]] == 0 {
            let left_state = (cur_step + 1, left_pos);
            if !state_set.contains(&left_state) {
                state_vec.push_back(left_state);
                state_set.insert(left_state);
            }
        }
        let up_pos = (cur_pos.0 - 1, cur_pos.1);
        if up_pos.0 >= 0 && cur_grid[[up_pos.0 as usize, up_pos.1 as usize]] == 0 {
            let up_state = (cur_step + 1, up_pos);
            // if !state_set.contains(&up_state) {
                state_vec.push_back(up_state);
                state_set.insert(up_state);
            // }
        }
        // Do nothing, if possible
        if cur_grid[[cur_pos.0 as usize, cur_pos.1 as usize]] == 0 {
            let stay_state = (cur_step + 1, cur_pos);
            if !state_set.contains(&stay_state) {
                state_vec.push_back(stay_state);
                state_set.insert(stay_state);
            }
        }
    }

    println!("Did not find goal!");
    (-1, init_blizzard_state.clone())
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day24/test_input.txt" };

    // Parse the input
    let (grid, init_blizzard_state) = parse_data(filename);

    // Part 1
    let (num_steps, _) = search_blizzard(&grid, &init_blizzard_state, true);
    println!("\nPart 1: Total steps = {}\n", num_steps);

    // Part 2
    let (steps_there, blizzard_state) = search_blizzard(&grid, &init_blizzard_state, true);
    let (steps_back, blizzard_state) = search_blizzard(&grid, &blizzard_state, false);
    let (steps_there_again, _) = search_blizzard(&grid, &blizzard_state, true);
    println!("\nPart 2: Total steps = {} + {} + {} = {}\n",
        steps_there, steps_back, steps_there_again,
        steps_there + steps_back + steps_there_again);
}

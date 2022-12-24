// Solution to Day 23 puzzle
// https://adventofcode.com/2022/day/23
//
// Example usage:
//   cargo run -r --bin day23 data/day23/test_input.txt

use std::env;
use std::fs;

// Handy type aliases
type Position = (i32, i32);
type State = Vec<Position>;

// Read the file and get initial elf positions
fn parse_data(filename: &str) -> State {
    let data = fs::read_to_string(filename).unwrap();
    let mut init_state = State::new();
    let lines = data.lines().collect::<Vec<&str>>();
    for (row, line) in lines.iter().enumerate() {
        for (col, elem) in line.chars().enumerate() {
            if elem == '#' {
                init_state.push((row as i32, col as i32));
            }
        }
    }
    init_state
}

// Prints the state of the map
fn display_state(state: &State) {
    let min_row = state.iter().map(|p| p.0).min().unwrap();
    let max_row = state.iter().map(|p| p.0).max().unwrap();
    let min_col = state.iter().map(|p| p.1).min().unwrap();
    let max_col = state.iter().map(|p| p.1).max().unwrap();
    let n_rows = max_row - min_row + 1;
    let n_cols = max_col - min_col + 1;

    let mut display_str = String::new();
    for r in 0..n_rows {
        for c in 0..n_cols {
            let test_position = (r as i32 + min_row as i32,
                                 c as i32 + min_col as i32);
            if state.contains(&test_position) {
                display_str.push_str("#");
            } else {
                display_str.push_str(".");
            }
        }
        display_str.push_str("\n");
    }
    println!("\nState:\n{}", display_str);
    println!("Rectangle size: {}\n", get_coverage_area(&state));
}

// Get the number of empty tiles in the rectangle spanning the elf positions
fn get_coverage_area(state: &State) -> u32 {
    let min_row = state.iter().map(|p| p.0).min().unwrap();
    let max_row = state.iter().map(|p| p.0).max().unwrap();
    let min_col = state.iter().map(|p| p.1).min().unwrap();
    let max_col = state.iter().map(|p| p.1).max().unwrap();
    let n_rows = max_row - min_row + 1;
    let n_cols = max_col - min_col + 1;

    let rectangle_size = (n_rows * n_cols) as u32;
    let num_elves = state.len() as u32;
    rectangle_size - num_elves
}

// Main simulation function for Part 1
fn simulate(init_state: &State, n_rounds: usize) -> u32 {

    let mut cur_state = init_state.clone();
    let mut directions = vec!['N', 'S', 'W', 'E'];

    for round in 1..=n_rounds {
        // println!("Round {}", round);
        let mut proposed_state = State::new();
        for cur_elf in &cur_state {
            // Get test points in all directions
            let pos_n = (cur_elf.0 - 1, cur_elf.1);
            let test_n = cur_state.contains(&pos_n);
            let pos_ne = (cur_elf.0 - 1, cur_elf.1 + 1);
            let test_ne = cur_state.contains(&pos_ne);
            let pos_nw = (cur_elf.0 - 1, cur_elf.1 - 1);
            let test_nw = cur_state.contains(&pos_nw);
            let pos_s = (cur_elf.0 + 1, cur_elf.1);
            let test_s = cur_state.contains(&pos_s);
            let pos_se = (cur_elf.0 + 1, cur_elf.1 + 1);
            let test_se = cur_state.contains(&pos_se);
            let pos_sw = (cur_elf.0 + 1, cur_elf.1 - 1);
            let test_sw = cur_state.contains(&pos_sw);
            let pos_e = (cur_elf.0, cur_elf.1 + 1);
            let test_e = cur_state.contains(&pos_e);
            let pos_w = (cur_elf.0, cur_elf.1 - 1);
            let test_w = cur_state.contains(&pos_w);

            let mut found_dir = false;
            for dir in &directions {
                // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
                if !(test_n || test_ne || test_nw || test_s || test_se || test_sw || test_e || test_w) {
                    break;
                }
                // Otherwise, the Elf looks in each of four directions in the following order
                // and proposes moving one step in the first valid direction.
                match dir {
                    'N' => {
                        // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
                        if !(test_n || test_ne || test_nw) {
                            proposed_state.push(pos_n);
                            found_dir = true;
                            break;
                        }
                    },
                    'S' => {
                        // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                        if !(test_s || test_se || test_sw) {
                            proposed_state.push(pos_s);
                            found_dir = true;
                            break;
                        }
                    },
                    'W' => {
                        // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
                        if !(test_w || test_nw || test_sw) {
                            proposed_state.push(pos_w);
                            found_dir = true;
                            break;
                        }
                    },
                    'E' => {
                        // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
                        if !(test_e || test_ne || test_se) {
                            proposed_state.push(pos_e);
                            found_dir = true;
                            break;
                        }
                    },
                    _ => {} // Should not be here
                }
            }
            // If we did not move in any direction, keep the current position
            if !found_dir {
                proposed_state.push(cur_elf.clone());
            }
        }

        // Now check if each elf can move by checking
        let mut new_state = State::new();
        for idx in 0..cur_state.len() {
            let test_pos = proposed_state[idx];
            let count = proposed_state.iter()
                        .filter(|&elem| *elem == test_pos)
                        .count();
            if count == 1 {
                new_state.push(test_pos);
            } else {
                new_state.push(cur_state[idx]);
            }
        }

        // Update
        directions.push(directions[0]);
        directions.remove(0);
        // display_state(&new_state);
        cur_state = new_state.clone();
    }

    let final_area = get_coverage_area(&cur_state);
    final_area
}

// Main simulation function for Part 2
fn simulate_until_done(init_state: &State) -> u32 {

    let mut cur_state = init_state.clone();
    let mut directions = vec!['N', 'S', 'W', 'E'];

    let mut round = 1;
    loop {
        // println!("Round {}", round);
        let mut proposed_state = State::new();
        for cur_elf in &cur_state {
            // Get test points in all directions
            let pos_n = (cur_elf.0 - 1, cur_elf.1);
            let test_n = cur_state.contains(&pos_n);
            let pos_ne = (cur_elf.0 - 1, cur_elf.1 + 1);
            let test_ne = cur_state.contains(&pos_ne);
            let pos_nw = (cur_elf.0 - 1, cur_elf.1 - 1);
            let test_nw = cur_state.contains(&pos_nw);
            let pos_s = (cur_elf.0 + 1, cur_elf.1);
            let test_s = cur_state.contains(&pos_s);
            let pos_se = (cur_elf.0 + 1, cur_elf.1 + 1);
            let test_se = cur_state.contains(&pos_se);
            let pos_sw = (cur_elf.0 + 1, cur_elf.1 - 1);
            let test_sw = cur_state.contains(&pos_sw);
            let pos_e = (cur_elf.0, cur_elf.1 + 1);
            let test_e = cur_state.contains(&pos_e);
            let pos_w = (cur_elf.0, cur_elf.1 - 1);
            let test_w = cur_state.contains(&pos_w);

            let mut found_dir = false;
            for dir in &directions {
                // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
                if !(test_n || test_ne || test_nw || test_s || test_se || test_sw || test_e || test_w) {
                    break;
                }
                // Otherwise, the Elf looks in each of four directions in the following order
                // and proposes moving one step in the first valid direction.
                match dir {
                    'N' => {
                        // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
                        if !(test_n || test_ne || test_nw) {
                            proposed_state.push(pos_n);
                            found_dir = true;
                            break;
                        }
                    },
                    'S' => {
                        // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                        if !(test_s || test_se || test_sw) {
                            proposed_state.push(pos_s);
                            found_dir = true;
                            break;
                        }
                    },
                    'W' => {
                        // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
                        if !(test_w || test_nw || test_sw) {
                            proposed_state.push(pos_w);
                            found_dir = true;
                            break;
                        }
                    },
                    'E' => {
                        // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
                        if !(test_e || test_ne || test_se) {
                            proposed_state.push(pos_e);
                            found_dir = true;
                            break;
                        }
                    },
                    _ => {} // Should not be here
                }
            }
            // If we did not move in any direction, keep the current position
            if !found_dir {
                proposed_state.push(cur_elf.clone());
            }
        }

        // Now check if each elf can move by checking
        let mut new_state = State::new();
        for idx in 0..cur_state.len() {
            let test_pos = proposed_state[idx];
            let count = proposed_state.iter()
                        .filter(|&elem| *elem == test_pos)
                        .count();
            if count == 1 {
                new_state.push(test_pos);
            } else {
                new_state.push(cur_state[idx]);
            }
        }

        // Check if we're done
        let mut no_elves_moved = true;
        for elf_idx in 0..cur_state.len() {
            if cur_state[elf_idx].0 != new_state[elf_idx].0 || cur_state[elf_idx].1 != new_state[elf_idx].1 {
                no_elves_moved = false;
                break;
            }
        }
        if no_elves_moved {
            break;
        }

        // Update
        directions.push(directions[0]);
        directions.remove(0);
        // display_state(&new_state);
        cur_state = new_state.clone();
        round += 1;
    }

    round
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day23/test_input.txt" };

    // Parse the input
    let init_state = parse_data(filename);
    // display_state(&init_state);

    // Part 1
    let final_coverage_area = simulate(&init_state, 10);
    println!("\nPart 1: Rectangle size = {}\n", final_coverage_area);

    // Part 2
    let final_round = simulate_until_done(&init_state);
    println!("\nPart 2: Final round = {}\n", final_round);
}

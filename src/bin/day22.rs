// Solution to Day 22 puzzle
// https://adventofcode.com/2022/day/22
//
// Example usage:
//   cargo run --bin day22 data/day22/test_input.txt

use std::env;
use std::fs;

extern crate itertools;
use itertools::Itertools;

extern crate ndarray;
use ndarray::Array2;


type State = (usize, usize, char);

fn parse_data(filename: &str) -> (Array2::<usize>, Vec<String>, State) {
    // Read the file and split it into the two parts
    let raw_text = fs::read_to_string(filename).unwrap();
    let (map_data, path_data) = raw_text.split("\n\n").next_tuple().unwrap();

    // The first part becomes a map
    // 0: undefined
    // 1: free
    // 2: wall
    let lines = map_data.lines().collect::<Vec<&str>>();
    let n_rows = lines.len();
    let n_cols = lines.iter().map(|line| line.len()).max().unwrap();
    let mut map = Array2::<usize>::zeros((n_rows, n_cols));
    for (row, line) in lines.iter().enumerate() {
        for (col, elem) in line.chars().enumerate() {
            map[[row, col]] = match elem {
                ' ' => 0,
                '.' => 1,
                '#' => 2,
                 _  => 3,  // Should not happen
            };
        }
    }
    // Also, find the initial position
    let init_row = 0;
    let init_head = '>';
    let mut init_col = 0;
    for col in 0..n_cols {
        if map[[0, col]] == 1 {
            init_col = col;
            break;
        }
    }
    let init_pos = (init_row, init_col, init_head);

    // The second part becomes a path
    // This was shamelessly ripped from this Stack Overflow question:
    // https://stackoverflow.com/questions/32257273/split-a-string-keeping-the-separators
    let mut path = Vec::new();
    let mut last = 0;
    for (index, matched) in path_data.match_indices(|c: char| !(c.is_numeric())) {
        if last != index {
            path.push((&path_data[last..index]).to_string());
        }
        path.push(matched.to_string());
        last = index + matched.len();
    }
    if last < path_data.len() {
        path.push((&path_data[last..]).to_string());
    }

    (map, path, init_pos)
}

fn simulate(map: &Array2::<usize>, path: &Vec<String>, init_pos: State) -> (State, u32) {
    let shape = map.shape();
    let n_rows = shape[0];
    let n_cols = shape[1];

    let mut state = init_pos.clone();
    for step in path.iter() {
        // println!("Executing step {}", step);

        if let Ok(n_steps) = step.parse::<u32>() {
            // If the step is numeric, keep moving
            for i in 0..n_steps {
                let (row, col, head) = state;
                let mut new_state = state;
                if head == '>' {
                    new_state = (row, col+1, head);
                    // Wrap around to right
                    if new_state.1 > n_cols - 1 || map[[new_state.0, new_state.1]] == 0 {
                        for c in 0..n_cols {
                            if map[[new_state.0, c]] != 0 {
                                new_state = (row, c, head);
                                break;
                            }
                        }
                    }
                } else if head == '<' {
                    new_state = (row, col-1, head);
                    // Wrap around to left
                    if col == 0 || map[[new_state.0, new_state.1]] == 0 {
                        for c in (0..n_cols).rev() {
                            if map[[new_state.0, c]] != 0 {
                                new_state = (row, c, head);
                                break;
                            }
                        }
                    }
                } else if head == '^' {
                    new_state = (row-1, col, head);
                    // Wrap around to top
                    if row == 0 || map[[new_state.0, new_state.1]] == 0 {
                        for r in (0..n_rows).rev() {
                            if map[[r, new_state.1]] != 0 {
                                new_state = (r, col, head);
                                break;
                            }
                        }
                    }
                } else if head == 'v' {
                    new_state = (row+1, col, head);
                    // Wrap around to bottom
                    if new_state.0 > n_rows - 1 || map[[new_state.0, new_state.1]] == 0 {
                        for r in 0..n_rows {
                            if map[[r, new_state.1]] != 0 {
                                new_state = (r, col, head);
                                break;
                            }
                        }
                    }
                }

                // Handle the wall condition
                if map[[new_state.0, new_state.1]] == 2 {
                    break;
                }
                state = new_state;
            }
        } else {
            // Otherwise, turn in place
            let (row, col, head) = state;
            if step == "L" {
                if head == '>' {
                    state = (row, col, '^');
                } else if head == '<' {
                    state = (row, col, 'v');
                } else if head == '^' {
                    state = (row, col, '<');
                } else if head == 'v' {
                    state = (row, col, '>');
                }
            } else if step == "R" {
                if head == '>' {
                    state = (row, col, 'v');
                } else if head == '<' {
                    state = (row, col, '^');
                } else if head == '^' {
                    state = (row, col, '>');
                } else if head == 'v' {
                    state = (row, col, '<');
                }
            }
        }

        // println!("\tState is now {:?}", state);
    }

    // Return the password from the final state, adding 1 to the row/column to one-index
    let final_state = (state.0 + 1, state.1 + 1, state.2);
    let heading_score = match state.2 {
        '>' => 0,
        'v' => 1,
        '<' => 2,
        '^' => 3,
        _   => 4, // Should not happen
    };
    let password = (final_state.0 as u32) * 1000 +
                   (final_state.1 as u32) * 4 +
                   heading_score;

    (final_state, password)
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day22/test_input.txt" };

    // Parse the input
    let (map, path, init_pos) = parse_data(filename);
    println!("Map:\n{:?}\n\nInit Pos: {:?}\n\nPath: {:?}\n", map, init_pos, path);

    // Part 1
    let (final_pos, password) = simulate(&map, &path, init_pos);
    println!("\nPart 1: Final position = {:?}, Password = {}\n", final_pos, password);

    // Part 2 : TBD?
}

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


type State = (i32, i32, char);

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
    let init_pos = (init_row as i32, init_col as i32, init_head);

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
    let n_rows = shape[0] as i32;
    let n_cols = shape[1] as i32;

    let mut state = init_pos.clone();
    for step in path.iter() {
        // println!("Executing step {}", step);

        if let Ok(n_steps) = step.parse::<u32>() {
            // If the step is numeric, keep moving
            for _ in 0..n_steps {
                let (row, col, head) = state;
                let mut new_state = state;
                if head == '>' {
                    new_state = (row, col+1, head);
                    // Wrap around to right
                    if new_state.1 > n_cols - 1 || map[[new_state.0 as usize, new_state.1 as usize]] == 0 {
                        for c in 0..n_cols {
                            if map[[new_state.0 as usize, c as usize]] != 0 {
                                new_state = (row, c, head);
                                break;
                            }
                        }
                    }
                } else if head == '<' {
                    new_state = (row, col-1, head);
                    // Wrap around to left
                    if col == 0 || map[[new_state.0 as usize, new_state.1 as usize]] == 0 {
                        for c in (0..n_cols).rev() {
                            if map[[new_state.0 as usize, c as usize]] != 0 {
                                new_state = (row, c, head);
                                break;
                            }
                        }
                    }
                } else if head == '^' {
                    new_state = (row-1, col, head);
                    // Wrap around to top
                    if row == 0 || map[[new_state.0 as usize, new_state.1 as usize]] == 0 {
                        for r in (0..n_rows).rev() {
                            if map[[r as usize, new_state.1 as usize]] != 0 {
                                new_state = (r, col, head);
                                break;
                            }
                        }
                    }
                } else if head == 'v' {
                    new_state = (row+1, col, head);
                    // Wrap around to bottom
                    if new_state.0 > n_rows - 1 || map[[new_state.0 as usize, new_state.1 as usize]] == 0 {
                        for r in 0..n_rows {
                            if map[[r as usize, new_state.1 as usize]] != 0 {
                                new_state = (r, col, head);
                                break;
                            }
                        }
                    }
                }

                // Handle the wall condition
                if map[[new_state.0 as usize, new_state.1 as usize]] == 2 {
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


fn simulate_puzzle_cube(map: &Array2::<usize>, path: &Vec<String>, init_pos: State) -> (State, u32) {
    // Specific to the puzzle input layout
    //        _____ _____
    //       |     |     |
    //       |  1  |  2  |
    //       |_____|_____|
    //       |     |
    //       |  3  |
    //  _____|_____|
    // |     |     |
    // |  4  |  5  |
    // |_____|_____|
    // |     |
    // |  6  |
    // |_____|
         

    let shape = map.shape();
    let n_rows = shape[0] as i32;
    let n_cols = shape[1] as i32;

    let mut state = init_pos.clone();
    for step in path.iter() {
        // println!("Executing step {}", step);

        if let Ok(n_steps) = step.parse::<u32>() {
            // If the step is numeric, keep moving
            for _ in 0..n_steps {
                let (row, col, head) = state;
                let mut new_state = state;
                if head == '>' {
                    new_state = (row, col+1, head);
                    // Wrap around on right
                    // Face 2 right -> Face 5 right
                    if (new_state.0 < 50) && (new_state.1 >= n_cols) {
                        new_state = (149 - row,
                                     99,
                                     '<');
                    }
                    // Face 3 right -> Face 2 down
                    else if (new_state.0 >= 50) && (new_state.0 < 100) && (new_state.1 >= 100) {
                        new_state = (49,
                                     100 + (row - 50),
                                     '^');
                    }
                    // Face 5 right -> Face 2 right
                    else if (new_state.0 >= 100) && (new_state.0 < 150) && (new_state.1 >= 100) {
                        new_state = (49 - (row - 100),
                                    (n_cols - 1),
                                     '<');
                    }
                    // Face 6 right -> Face 5 down
                    else if (new_state.0 >= 150) && (new_state.0 < 200) && (new_state.1 >= 50) {
                        new_state = (149,
                                     50 + (row - 150),
                                     '^');
                    }
                    
                } else if head == '<' {
                    new_state = (row, col-1, head);
                    // Wrap around on left
                    // Face 1 left -> Face 4 left
                    if (new_state.0 < 50) && (new_state.1 < 50) {
                        new_state = (149 - row,
                                     0,
                                     '>');
                    }
                    // Face 3 left -> Face 4 up
                    else if (new_state.0 >= 50) && (new_state.0 < 100) && (new_state.1 < 50) {
                        new_state = (100,
                                     row - 50,
                                     'v');
                    }
                    // Face 4 left -> Face 1 left
                    else if (new_state.0 >= 100) && (new_state.0 < 150) && (new_state.1 < 0) {
                        new_state = (49 - (row - 100),
                                     50,
                                     '>');
                    }
                    // Face 6 left -> Face 1 up
                    else if (new_state.0 >= 150) && (new_state.0 < 200) && (new_state.1 < 0) {
                        new_state = (0,
                                     (row - 150) + 50,
                                     'v');
                    }

                } else if head == '^' {
                    new_state = (row-1, col, head);
                    // Wrap around on top
                    // Face 1 up -> Face 6 left
                    if (new_state.0 < 0) && (new_state.1 >= 50) && (new_state.1 < 100) {
                        new_state = (150 + (col - 50),
                                     0,
                                     '>');
                    }
                    // Face 2 up -> Face 6 down
                    else if (new_state.0 < 0) && (new_state.1 >= 100) && (new_state.1 < 150) {
                        new_state = (n_rows - 1,
                                     col - 100,
                                     '^');
                    }
                    // Face 4 up -> Face 3 left
                    else if (new_state.0 < 100) && (new_state.1 < 50) {
                        new_state = (50 + col,
                                     50,
                                     '>');
                    }

                } else if head == 'v' {
                    new_state = (row+1, col, head);
                    // Wrap around to bottom
                    // Face 2 down -> Face 3 right
                    if (new_state.0 >= 50) && (new_state.1 >= 100) && (new_state.1 < 150) {
                        new_state = (50 + (col - 100),
                                     99,
                                     '<');
                    }
                    // Face 5 down -> Face 6 right
                    else if (new_state.0 >= 150) && (new_state.1 >= 50) && (new_state.1 < 100) {
                        new_state = (150 + (col - 50),
                                     49,
                                     '<');
                    }
                    // Face 6 down -> Face 2 up
                    else if (new_state.0 >= 200) && (new_state.1 < 50) {
                        new_state = (0,
                                     100 + col,
                                     'v');
                    }
                }

                // Handle the wall condition
                if map[[new_state.0 as usize, new_state.1 as usize]] == 2 {
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
    println!("Map:\n{:?}\n\nInit Pos: {:?}\n", map, init_pos);

    // Part 1
    let (final_pos, password) = simulate(&map, &path, init_pos);
    println!("\nPart 1: Final position = {:?}, Password = {}\n", final_pos, password);

    // Part 2 : Only works for puzzle input!
    let (final_cube_pos, cube_password) = simulate_puzzle_cube(&map, &path, init_pos);
    println!("\nPart 2: Final cube position = {:?}, Password = {}\n", final_cube_pos, cube_password);
}

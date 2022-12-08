// Solution to Day 8 puzzle
// https://adventofcode.com/2022/day/8
//
// Example usage:
//   cargo run --bin day8 test_input.txt

use std::cmp::max;
use std::env;
use std::fs;

// Visibility check function for part 1
fn check_visibility(data: &Vec<Vec<u32>>, 
                    n_rows: usize, n_cols: usize,
                    row: usize, col: usize) -> bool {
    if row==0 || row==(n_rows-1) || col==0 || col==(n_cols-1) {
        // If the tree is in a corner, it is visible
        return true;
    } else {
        let height = data[row][col];
        // TOP
        let mut all_lower = true;
        for n in 0..row {
            all_lower = all_lower && (data[n][col] < height);
        }
        if all_lower {
            return true;
        }
        // BOTTOM
        let mut all_lower = true;
        for n in row+1..n_rows {
            all_lower = all_lower && (data[n][col] < height);
        }
        if all_lower {
            return true;
        }
        // LEFT
        let mut all_lower = true;
        for n in 0..col {
            all_lower = all_lower && (data[row][n] < height);
        }
        if all_lower {
            return true;
        }
        // RIGHT
        let mut all_lower = true;
        for n in col+1..n_cols {
            all_lower = all_lower && (data[row][n] < height);
        }
        if all_lower { return true; }
        return false;
    }
}

// Scenic score function for Part 2
fn scenic_score(data: &Vec<Vec<u32>>, 
                n_rows: usize, n_cols: usize,
                row: usize, col: usize) -> u32 {
    println!("Check scenic score for ({}, {}) = {}", row, col, data[row][col]);
    let height = data[row][col];
    // TOP
    let mut top_scenic_score = 0;
    for n in (0..row).rev() {
        top_scenic_score += 1;
        if data[n][col] >= height {
            break;
        }
    }
    // BOTTOM
    let mut bottom_scenic_score = 0;
    for n in row+1..n_rows {
        bottom_scenic_score += 1;
        if data[n][col] >= height {
            break;
        }
    }
    // LEFT
    let mut left_scenic_score = 0;
    for n in (0..col).rev() {
        left_scenic_score += 1;
        if data[row][n] >= height {
            break;
        }
    }
    // RIGHT
    let mut right_scenic_score = 0;
    for n in col+1..n_cols {
        right_scenic_score += 1;
        if data[row][n] >= height {
            break;
        }
    }

    println!("Top: {}, Bottom: {}, Left: {}, Right: {}",
        top_scenic_score, bottom_scenic_score, left_scenic_score, right_scenic_score);
    top_scenic_score * bottom_scenic_score * left_scenic_score * right_scenic_score
}


fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day8/test_input.txt" };

    // Read the file and convert it to a vector of vectors
    let data_raw = fs::read_to_string(filename).unwrap();
    let mut data = Vec::new();
    for line in data_raw.lines() {
        data.push(
            line.chars()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        );
    }
    let n_rows = data.len();
    let n_cols = data[0].len();

    // PART 1: Go through the data and find all visible trees
    let mut visible_trees = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            if check_visibility(&data, n_rows, n_cols, i, j) {
                visible_trees += 1;
            }
        }
    } 
    println!("\nPart 1 Total visible trees: {}\n", visible_trees);

    // PART 2: Go through the data and find max scenic score
    let mut max_scenic_score = 0;
    for i in 0..n_rows {
        for j in 0..n_cols {
            let score = scenic_score(&data, n_rows, n_cols, i, j);
            max_scenic_score = max(score, max_scenic_score);
        }
    } 
    println!("\nPart 2 Max scenic score: {}\n", max_scenic_score);


}

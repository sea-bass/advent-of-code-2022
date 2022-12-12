// Solution to Day 12 puzzle
// https://adventofcode.com/2022/day/12
//
// Example usage:
//   cargo run --bin day12 data/day12/test_input.txt

use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

// Node structure
// Position is (X, Y) assuming the origin is upper left:
// 
//   o---> X
//   |
//   v
//   Y
//
struct Node {
    pos: (i32, i32),
    val: char,
    cost: u32
}

// Helper functions
fn is_valid(pos: (i32, i32), size: (i32, i32)) -> bool {
    let valid_x = pos.0 >= 0 && pos.0 < size.0;
    let valid_y = pos.1 >= 0 && pos.1 < size.1;
    valid_x && valid_y
}

fn height_value(val: char) -> u8 {
    match val {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _ => val as u8,
    }
}

fn search(data: &Vec<Vec<char>>, maybe_start_pos: Option<(i32, i32)>) -> u32 {
    // Initialize data structures
    let mut expanded = HashSet::new();
    let mut queue = VecDeque::new();

    // If not specified the starting pose
    let grid_size = (data[0].len() as i32, data.len() as i32);
    let start_pos = match maybe_start_pos {
        Some(pos) => pos,
        None => {
            let mut spos: (i32, i32) = (0, 0);
            for row in 0..data.len() {
                for col in 0..data[row].len() {
                    if data[row][col] == 'S' {
                        spos = (col as i32, row as i32);
                    }
                }
            }
            spos
        }
    };
    queue.push_back(
        Node { 
            pos: start_pos,
            val: data[start_pos.1 as usize][start_pos.0 as usize],
            cost: 0 
        }
    );

    // Search 
    let mut n_steps = u32::MAX;
    loop {
        // Get an element from the queue
        if queue.is_empty() {
            println!("Queue is empty. No solution found!");
            break;
        }
        let elem = queue.pop_front().unwrap();
        let elem_value = height_value(elem.val);
        expanded.insert(elem.pos);

        // Done criteria
        if elem.val == 'E' {
            n_steps = elem.cost;
            break;
        }

        // Get neighbors
        let right_pos = (elem.pos.0 + 1, elem.pos.1);
        if is_valid(right_pos, grid_size) && !expanded.contains(&right_pos) {
            let val = data[right_pos.1 as usize][right_pos.0 as usize];
            if height_value(val) <= elem_value + 1 {
                queue.push_back(Node {pos: right_pos, val, cost: elem.cost + 1});
                expanded.insert(right_pos);
            }
        }
        let left_pos = (elem.pos.0 - 1, elem.pos.1);
        if is_valid(left_pos, grid_size) && !expanded.contains(&left_pos) {
            let val = data[left_pos.1 as usize][left_pos.0 as usize];
            if height_value(val) <= elem_value + 1 {
                queue.push_back(Node {pos: left_pos, val, cost: elem.cost + 1});
                expanded.insert(left_pos);
            }
        }
        let down_pos = (elem.pos.0, elem.pos.1 + 1);
        if is_valid(down_pos, grid_size) && !expanded.contains(&down_pos) {
            let val = data[down_pos.1 as usize][down_pos.0 as usize];
            if height_value(val) <= elem_value + 1 {
                queue.push_back(Node {pos: down_pos, val, cost: elem.cost + 1});
                expanded.insert(down_pos);
            }
        }
        let up_pos = (elem.pos.0, elem.pos.1 - 1);
        if is_valid(up_pos, grid_size) && !expanded.contains(&up_pos) {
            let val = data[up_pos.1 as usize][up_pos.0 as usize];
            if height_value(val) <= elem_value + 1 {
                queue.push_back(Node {pos: up_pos, val, cost: elem.cost + 1});
                expanded.insert(up_pos);
            }
        }
    }
    n_steps
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day12/test_input.txt" };

    // Read the file and convert it to a vector of vectors
    let data_raw = fs::read_to_string(filename).unwrap();
    let mut data = Vec::new();
    for line in data_raw.lines() {
        data.push(
            line.chars()
            .collect::<Vec<char>>()
        );
    }

    // PART 1: Search from S.
    let n_steps = search(&data, None);
    println!("\nPart 1 Total steps: {}\n", n_steps);

    // PART 2: Search from all 'a' positions.
    let mut min_n_steps = u32::MAX;
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 'a' {
                let n_steps = search(&data, Some((col as i32, row as i32)));
                min_n_steps = min(n_steps, min_n_steps);
            }
        }
    }
    println!("\nPart 2 Min total steps: {}\n", min_n_steps);
}

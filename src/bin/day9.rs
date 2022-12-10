// Solution to Day 9 puzzle
// https://adventofcode.com/2022/day/9
//
// Example usage:
//   cargo run --bin day9 data/day9/test_input.txt

use std::collections::HashSet;
use std::env;
use std::fs;

type Position = (i32, i32);

fn get_head_position(head: Position, dir: &str) -> Position {
    let mut new_head = head;
    if dir == "R" {
        new_head.0 += 1;
    } else if dir == "L" {
        new_head.0 -= 1;
    } else if dir == "U" {
        new_head.1 += 1;
    } else if dir == "D" {
        new_head.1 -= 1;
    }
    new_head
}

fn get_tail_position(head: Position, tail: Position) -> Position {

    let mut new_tail = tail;
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;
    let distance = x_diff.abs() + y_diff.abs();
    if y_diff == 0 && distance > 1 {
        // Follow to the left/right
        new_tail.0 += x_diff.signum();
    } else if x_diff == 0 && distance > 1 {
        // Follow to the top/bottom
        new_tail.1 += y_diff.signum();
    } else if distance > 2 {
        // Diagonal moves if head is far enough away
        new_tail.0 += x_diff.signum();
        new_tail.1 += y_diff.signum();
    }
    new_tail
}

fn part1(filename: &str) {
    // Read the file
    let data = fs::read_to_string(filename).unwrap();

    // Go through all the moves
    let mut tail_pos_map = HashSet::new();
    let mut cur_head_pos: Position = (0, 0);
    let mut cur_tail_pos: Position = (0, 0);
    tail_pos_map.insert(cur_tail_pos.clone());

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = parts[0];
        let num_steps = parts[1].parse::<u32>().unwrap();

        for _ in 0..num_steps {
            // Move the head
            cur_head_pos = get_head_position(cur_head_pos, dir);
            
            // Move the tail
            cur_tail_pos = get_tail_position(cur_head_pos, cur_tail_pos);
            tail_pos_map.insert(cur_tail_pos);
        }

    }
    
    let num_positions_visited = tail_pos_map.len();
    println!("\nPart 1: Positions visited: {}\n", num_positions_visited);
}

fn part2(filename: &str, num_segments: usize) {
    // Read the file
    let data = fs::read_to_string(filename).unwrap();

    // Initialize data structures for all the segments
    let init_pos = (0, 0);
    let mut tail_pos_map = HashSet::new();
    let mut positions = Vec::with_capacity(num_segments);
    for _ in 0..num_segments {
        positions.push(init_pos);
    }
    tail_pos_map.insert(init_pos);

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = parts[0];
        let num_steps = parts[1].parse::<u32>().unwrap();

        for _ in 0..num_steps {
            // Move the head
            positions[0] = get_head_position(positions[0], dir);
            
            // Move the rest of the rope in order
            for i in 1..num_segments {
                positions[i] = get_tail_position(positions[i-1], positions[i]);
            }
            tail_pos_map.insert(*positions.last().unwrap());
        }

    }
    
    let num_positions_visited = tail_pos_map.len();
    println!("\nPart 2: Positions visited for {} segment rope: {}\n",
        num_segments, num_positions_visited);
}


fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day9/test_input_1.txt" };

    // Solve the puzzles!
    part1(filename);
    part2(filename, 10);
}

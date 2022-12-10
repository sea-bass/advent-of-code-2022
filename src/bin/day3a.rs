// Solution to Day 3 puzzle, Part 1
// https://adventofcode.com/2022/day/3
//
// Example usage:
//   cargo run --bin day3a data/day3/test_input.txt

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const LOWER_A_VAL: u32 = 'a' as u32;
const LOWER_Z_VAL: u32 = 'z' as u32;
const UPPER_A_VAL: u32 = 'A' as u32;
const UPPER_Z_VAL: u32 = 'Z' as u32;

fn get_item_priority(item: char) -> u32 {
    let item_val = item as u32;

    // If the item is lower case, priority is 1..26
    // If the item is upper case, priority is 27..52
    if item_val >= LOWER_A_VAL && item_val <= LOWER_Z_VAL {
        return item_val - LOWER_A_VAL + 1;
    } else if item_val >= UPPER_A_VAL && item_val <= UPPER_Z_VAL {
        return item_val - UPPER_A_VAL + 27;
    }

    // Otherwise, warn and return 0
    println!("Invalid item {}, returning zero priority", item);
    return 0;
}

fn get_rucksack_priority(line: String) -> u32 {
    // Split the line into two halves
    let len = line.len();
    let left = &line[0..len/2];
    let right = &line[len/2..len];
    println!("{}", line);

    // Get the first common occurrence
    for item in left.chars() {
        if right.contains(item) {
            let priority = get_item_priority(item);
            println!("Found common item {}, priority {}", item, priority);
            return priority;
        }
    }

    // We shouldn't get here due to the puzzle constraints, but...
    println!("Found no common item, returning zero priority.");
    return 0;
}

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day3/test_input.txt" };

    // Read the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Go through all the lines and tally up the priorities
    let mut total_priority = 0;
    for line in reader.lines() {
        total_priority += get_rucksack_priority(line?);
    }

    println!("\nTotal priority: {}\n", total_priority);

    Ok(())
}

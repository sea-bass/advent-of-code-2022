// Solution to Day 3 puzzle, Part 2
// https://adventofcode.com/2022/day/3
//
// Example usage:
//   cargo run --bin day3b data/day3/test_input.txt

use std::env;
use std::fs;

const GROUP_SIZE: usize = 3;
const LOWER_A_VAL: u32 = 'a' as u32;
const LOWER_Z_VAL: u32 = 'z' as u32;
const UPPER_A_VAL: u32 = 'A' as u32;
const UPPER_Z_VAL: u32 = 'Z' as u32;

// If the item is lower case, priority is 1..26
// If the item is upper case, priority is 27..52
fn get_item_priority(item: char) -> u32 {
    let item_val = item as u32;
    return match item_val {
        LOWER_A_VAL..=LOWER_Z_VAL => item_val - LOWER_A_VAL + 1,
        UPPER_A_VAL..=UPPER_Z_VAL => item_val - UPPER_A_VAL + 27,
        _ => {
            println!("Invalid item {}, returning zero priority", item);
            0
        }
    };
}

fn get_rucksack_priority(lines: &[&str]) -> u32 {
    for item in lines[0].chars() {
        // Check that all other elves in the group contain this item
        let mut is_badge = true;
        for other_line in &lines[1..] {
            if !other_line.contains(item) {
                is_badge = false;
                break;
            }
        }
        if is_badge {
            let priority = get_item_priority(item);
            println!("Found badge item: {}, priority {}", item, priority);
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
    let data = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    // Go through all the lines and tally up the priorities
    let mut total_priority = 0;
    for group_lines in lines.chunks(GROUP_SIZE) {
        total_priority += get_rucksack_priority(group_lines);
    }

    println!("\nTotal priority: {}\n", total_priority);

    Ok(())
}

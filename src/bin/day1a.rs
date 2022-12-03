// Solution to Day 1 puzzle, Part 1
// https://adventofcode.com/2022/day/1
//
// Example usage:
//   cargo run --bin day1a test_input.txt

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day1/test_input.txt" };

    // Read the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Keep a running count of calories and update the max accordingly.
    let mut index: i32 = 1;  // Assume 1-indexing
    let mut max_index: i32 = 1;
    let mut calorie_count: i32 = 0;
    let mut max_calories: i32 = -1;
    for line in reader.lines() {
        let line_text = line?;

        // If the line is empty, reset the running count; else increment it.
        if line_text.len() == 0 {
            calorie_count = 0;
            index += 1;
        } else {
            let cals = line_text.parse::<i32>().unwrap();
            calorie_count += cals;
        }

        // Keep track of the maximum calories so far and the corresponding index.
        if calorie_count > max_calories {
            max_calories = calorie_count;
            max_index = index;
        }

        // Debug print
        println!("\ttext: {}, curr idx {}, curr calories {}, max idx {}, max calories {}",
                 line_text, index, calorie_count, max_index, max_calories);
    }

    println!("Elf {} has the most calories ({})", max_index, max_calories);
    Ok(())
}

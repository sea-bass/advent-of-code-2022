// Solution to Day 4 puzzle, Part 1
// https://adventofcode.com/2022/day/4
//
// Example usage:
//   cargo run --bin day4a test_input.txt

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day4/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    // Go through all the lines and find redundant assignments.
    let mut num_redundant = 0;
    for line in data.split("\n") {
        let nums: Vec<u32> = line.split(&['-', ','][..])
            .map(|x| x.parse::<u32>().unwrap()).collect();

        let first_dominates = nums[0] <= nums[2] && nums[1] >= nums[3];
        let second_dominates = nums[0] >= nums[2] && nums[1] <= nums[3];
        if first_dominates || second_dominates {
            println!("Found dominating pair: {}", line);
            num_redundant += 1;
        }
    }

    println!("\nTotal redundant assignments: {}\n", num_redundant);

    Ok(())
}

// Solution to Day 4 puzzle, Part 2
// https://adventofcode.com/2022/day/4
//
// Example usage:
//   cargo run --bin day4b test_input.txt

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day4/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    // Go through all the lines and find overlapping assignments per line.
    let mut num_redundant = 0;
    for line in data.split("\n") {
        let nums: Vec<u32> = line.split(&['-', ','][..])
            .map(|x| x.parse::<u32>().unwrap()).collect();

        let first_below_second = nums[1] < nums[2];
        let first_above_second = nums[0] > nums[3];
        if !(first_below_second || first_above_second) {
            println!("Found overlapping pair: {}", line);
            num_redundant += 1;
        }
    }

    println!("\nTotal redundant assignments: {}\n", num_redundant);

    Ok(())
}

// Solution to Day 4 puzzle, Part 1
// https://adventofcode.com/2022/day/4
//
// Example usage:
//   cargo run --bin day4a data/day4/test_input.txt

use std::env;
use std::fs;

extern crate itertools;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day4/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();

    // Go through all the lines and find redundant assignments.
    let mut num_redundant = 0;
    for line in data.split("\n") {
        let (min1, max1, min2, max2) = line.split(&['-', ','][..])
            .map(|x| x.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();

        let first_dominates = min1 <= min2 && max1 >= max2;
        let second_dominates = min1 >= min2 && max1 <= max2;
        if first_dominates || second_dominates {
            println!("Found dominating pair: {}", line);
            num_redundant += 1;
        }
    }

    println!("\nTotal redundant assignments: {}\n", num_redundant);

    Ok(())
}

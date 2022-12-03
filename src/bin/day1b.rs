// Solution to Day 1 puzzle, Part 2
// https://adventofcode.com/2022/day/1
//
// Example usage:
//   cargo run --bin day1b test_input.txt

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

    // Keep a running count of calories for each elf.
    let mut cal_vec: Vec<i32> = Vec::new();
    let mut calorie_count: i32 = 0;
    for line in reader.lines() {
        let line_text = line?;

        // If the line is empty, reset the running count and push to the vector of elves;
        // else increment the count.
        if line_text.len() == 0 {
            cal_vec.push(calorie_count);
            calorie_count = 0;
        } else {
            let cals = line_text.parse::<i32>().unwrap();
            calorie_count += cals;
        }
    }

    // Get the top elves calorie values
    let num_top_elves = 3;
    let num_elves = cal_vec.len();
    cal_vec.sort();

    // Naive formulation which lets us print calories for each element
    let mut top_elves_calories = 0;
    for n in 1..=num_top_elves {
        let cals = cal_vec[num_elves - n];
        top_elves_calories += cals;
        println!("Calories for Top {} elf: {}", n, cals);
    }
    println!("Total calories for Top {} elves: {}", num_top_elves, top_elves_calories);

    // The alternative one liner
    let top_elves_calories_alt: i32 = cal_vec[num_elves - num_top_elves .. num_elves].iter().sum();
    println!("Alternative calculation: {}", top_elves_calories_alt);

    Ok(())
}

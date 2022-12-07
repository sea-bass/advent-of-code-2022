// Solution to Day 6 puzzle
// https://adventofcode.com/2022/day/6
//
// Example usage:
//   cargo run --bin day6 test_input.txt

use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;
use std::fs;

fn get_marker(chars : &Vec<char>, streak_length: usize) {
    for i in streak_length..=chars.len() {
        let set = HashSet::<&char>::from_iter(
            &chars[i-streak_length..i]
        );
        if set.len() == streak_length {
            println!("Found marker at index {}", i);
            break;
        }
    }    
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[2] } else { "data/day6/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();
    let chars = data.chars().collect::<Vec<char>>();

    //  Look for markers
    println!("PART 1:");
    get_marker(&chars, 4);
    println!("PART 2:");
    get_marker(&chars, 14);

}

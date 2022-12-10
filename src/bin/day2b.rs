// Solution to Day 2 puzzle, Part 2
// https://adventofcode.com/2022/day/2
//
// Example usage:
//   cargo run --bin day2b data/day2/test_input.txt

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_score(opponent: char, strategy: char) -> i32 {
    // Convert scores to ASCII
    let opponent_val = opponent as i32 - 'A' as i32 + 1;
    let win_val = strategy as i32 - 'Y' as i32;

    // Score for winning / drawing is based on the strategy provided
    let win_score;
    if win_val == 1 {
        win_score = 6; // win
    } else if win_val == -1 {
        win_score = 0; // lose
    } else {
        win_score = 3; // draw
    }

    // Based on the winning score, figure out the play and selection score
    let mut selection_score = opponent_val + win_val;
    if selection_score > 3 {
        selection_score -= 3;
    } else if selection_score < 1 {
        selection_score += 3;
    }

    let score = selection_score + win_score;
    println!("{} needs win {}, selection score: {}, win score: {}, score: {}",
        opponent, win_val, selection_score, win_score, score);
    return score;
}

fn main() -> std::io::Result<()> {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day2/test_input.txt" };

    // Read the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Go through all the lines and tally up the score
    let mut total_score: i32 = 0;
    for line in reader.lines() {
        let line_text = line?;
        let v: Vec<char> = line_text.chars().collect();
        total_score += get_score(v[0], v[2]);  // Assumes single space
    }

    println!("\nTotal score: {}\n", total_score);

    Ok(())
}

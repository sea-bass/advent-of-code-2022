// Solution to Day 10 puzzle
// https://adventofcode.com/2022/day/10
//
// Example usage:
//   cargo run --bin day10 data/day10/test_input.txt

use std::env;
use std::fs;

fn render(rendering: &mut String, sprite_pos: i32, cycle: i32, render_period: i32) {
    let pixel_pos = (cycle-1) % render_period;

    // Add line break if the render period was hit
    if pixel_pos == 0 {
        rendering.push_str("\n");
    }

    // If the sprite position coincides with the pixel
    if (pixel_pos >= sprite_pos - 1) && (pixel_pos <= sprite_pos + 1) {
        rendering.push_str("#");
    } else {
        rendering.push_str(".");
    }
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day10/test_input.txt" };

    // Constants
    const MEASURE_START: i32 = 20;
    const MEASURE_PERIOD: i32 = 40;
    const RENDER_STEP: i32 = 40;

    // Initial state
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut next_cycle: i32 = MEASURE_START;
    let mut signal_strength: i32 = 0;
    let mut rendering = "".to_string();

    // Loop through the data
    let data = fs::read_to_string(filename).unwrap();
    for line in data.lines() {
        if line == "noop" {
            render(&mut rendering, x, cycle, RENDER_STEP);
            cycle += 1;
            if cycle == next_cycle {
                signal_strength += x * next_cycle;
                next_cycle += MEASURE_PERIOD;
            }
        } else {  // addx
            let parts: Vec<&str> = line.split_whitespace().collect();
            let val = parts[1].parse::<i32>().unwrap();

            for i in 0..2 {
                render(&mut rendering, x, cycle, RENDER_STEP);
                cycle += 1;
                if i == 1 {
                    x += val;
                }
                if cycle == next_cycle {
                    signal_strength += x * next_cycle;
                    next_cycle += MEASURE_PERIOD;
                }
            }
        }
    }

    println!("\nPart 1: Final signal strength at cycle {}: {}", cycle, signal_strength);

    println!("\nPart 2: Rendering:\n{}", rendering);
}

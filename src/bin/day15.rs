// Solution to Day 15 puzzle
// https://adventofcode.com/2022/day/15
//
// Example usage:
//   cargo run --bin day15 <filename> <test row>
//   cargo run --bin day15 data/day15/test_input.txt 10
//   cargo run --bin day15 data/day15/puzzle_input.txt 2000000
//
// NOTE: For Part 1, <test_row> refers to the single row to check
//       For Part 2, <test_row> refers to the upper limit of the range 0..=<test_row>

use std::cmp::{min, max};
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fs;

fn distance(sensor: &(i32, i32), beacon: &(i32, i32)) -> u32 {
    ((sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()).try_into().unwrap()
}

fn search_row(data: &str, test_y: i32) -> u32 {
    let mut loc_map = HashMap::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut max_distance: u32 = 0;

    for line in data.lines() {

        // Pretty brute-force split, but it works for this problem
        let tokens = line.split(&['=', ':', ','][..]).collect::<Vec<&str>>();
        let sensor_x = tokens[1].parse::<i32>().unwrap();
        let sensor_y = tokens[3].parse::<i32>().unwrap();
        let beacon_x = tokens[5].parse::<i32>().unwrap();
        let beacon_y = tokens[7].parse::<i32>().unwrap();

        // Insert the location
        let sensor = (sensor_x, sensor_y);
        let beacon = (beacon_x, beacon_y);
        loc_map.insert(sensor, beacon);
        max_distance = max(max_distance, distance(&sensor, &beacon));

        // Update the bounds
        min_x = min(min_x, min(sensor_x, beacon_x));
        max_x = max(max_x, max(sensor_x, beacon_x));
        min_y = min(min_y, min(sensor_y, beacon_y));
        max_y = max(max_y, max(sensor_y, beacon_y));
    }

    let mut can_exist;
    let mut num_cannot_exist = 0;
    for test_x in min_x - (max_distance as i32)..=max_x + (max_distance as i32) {
        // println!("Testing column {}...", test_x);
        
        can_exist = true;
        for (&sensor, &beacon) in &loc_map {
            // First, check if the row is a beacon itself
            if beacon.0 == test_x && beacon.1 == test_y {
                // println!("  Found beacon!");
                break;
            }

            // Then, check with the actual sensor
            let radius = distance(&sensor, &beacon);
            // println!("  Testing sensor pair {:?} {:?}, max distance {}", sensor, beacon, max_distance);
            if distance(&sensor, &(test_x, test_y)) <= radius {
                // println!("  Cannot exist from sensor {} {}, distance {} < {}", 
                //     sensor.0, sensor.1, distance(sensor, (test_x, test_y)), radius);
                can_exist = false;
                break;
            }    
        }

        if !can_exist {
            num_cannot_exist += 1;
        }
    }

    num_cannot_exist
}

fn search_beacons(data: &str, max_dim: i32) -> ((i32, i32), u32) {
    let mut loc_map = HashMap::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut max_distance: u32 = 0;

    for line in data.lines() {

        // Pretty brute-force split, but it works for this problem
        let tokens = line.split(&['=', ':', ','][..]).collect::<Vec<&str>>();
        let sensor_x = tokens[1].parse::<i32>().unwrap();
        let sensor_y = tokens[3].parse::<i32>().unwrap();
        let beacon_x = tokens[5].parse::<i32>().unwrap();
        let beacon_y = tokens[7].parse::<i32>().unwrap();

        // Insert the location
        let sensor = (sensor_x, sensor_y);
        let beacon = (beacon_x, beacon_y);
        loc_map.insert(sensor, beacon);
        max_distance = max(max_distance, distance(&sensor, &beacon));

        // Update the bounds
        min_x = min(min_x, min(sensor_x, beacon_x));
        max_x = max(max_x, max(sensor_x, beacon_x));
        min_y = min(min_y, min(sensor_y, beacon_y));
        max_y = max(max_y, max(sensor_y, beacon_y));
    }

    let mut can_exist;
    let mut beacon_position: (i32, i32) = (-1, -1);
    let mut tuning_frequency: u32 = 0;
    for test_y in 0..=max_dim {
        println!("Testing row {} / {}", test_y, max_dim);
        for test_x in 0..=max_dim {            
            can_exist = true;
            for (&sensor, &beacon) in &loc_map {
                let radius = distance(&sensor, &beacon);
                if distance(&sensor, &(test_x, test_y)) <= radius {
                    can_exist = false;
                    break;
                }
            }

            if can_exist {
                // println!("Found the beacon at {} {}", test_x, test_y);
                beacon_position = (test_x, test_y);
                tuning_frequency = (test_x * 4000000 + test_y).try_into().unwrap();
                return (beacon_position, tuning_frequency);
            }
        }
    }

    (beacon_position, tuning_frequency)
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day15/test_input.txt" };
    let test_val: i32 = if args.len() > 2 { args[2].parse::<i32>().unwrap() } else { 10 };

    // Parse the file
    let data = fs::read_to_string(filename).unwrap();

    // Part 1
    let num_cannot_exist = search_row(&data, test_val);
    println!("\nPart 1: There are {} positions where beacon cannot exist\n", num_cannot_exist);

    // PART 2: 
    let (beacon_location, tuning_frequency) = search_beacons(&data, test_val);
    println!("\nPart 2: Beacon at {:?}, Frequency {}\n", beacon_location, tuning_frequency);
}

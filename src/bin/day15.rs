// Solution to Day 15 puzzle
// https://adventofcode.com/2022/day/15
//
// Usage:
//   cargo run --bin day15 <filename> <test_val>
//
// NOTE: For Part 1, <test_val> refers to the single row to check
//       For Part 2, <test_val> refers to the upper limit of the range 0..=<test_row>
//
// Since this puzzle takes a while, recommend running with the -r flag for release profile.
//
// To solve Part 1:
//   cargo run -r --bin day15 data/day15/test_input.txt 10
//   cargo run -r --bin day15 data/day15/puzzle_input.txt 2000000
//
// To solve Part 2:
//   cargo run -r --bin day15 data/day15/test_input.txt 20
//   cargo run -r --bin day15 data/day15/puzzle_input.txt 4000000

use std::cmp::{min, max};
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fs;

// Helper functions
fn distance(sensor: &(i32, i32), beacon: &(i32, i32)) -> u32 {
    ((sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()).try_into().unwrap()
}

fn in_range(pt: &(i32, i32), max_dim: i32) -> bool {
    pt.0 >= 0 && pt.0 <= max_dim && pt.1 >=0 && pt.1 <= max_dim
}

// Part 1 naive implementation
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

// Part 2 refactored implementation
fn search_beacons(data: &str, max_dim: i32) -> ((i32, i32), i64) {
    let mut loc_map = HashMap::new();
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
    }

    let mut beacon_position: (i32, i32) = (-1, -1);
    let mut tuning_frequency: i64 = 0;

    for (&sensor, &beacon) in &loc_map {
        // Get a list of points immediately surrounding the radius.
        // This works because the beacon must be 1 unit away from a sensor's range.
        let search_radius = (distance(&sensor, &beacon) as i32) + 1;
        let mut test_pts = Vec::new();
        for y in -search_radius..=search_radius {
            let x_offset = search_radius.abs() - y;

            let left_pt = (sensor.1 - x_offset, sensor.0 + y);
            if in_range(&left_pt, max_dim) {
                test_pts.push(left_pt);
            }

            // No need to add a right point if we're at the tip of the diamond
            if x_offset > 0 {
                let right_pt = (sensor.1 + x_offset, sensor.0 + y);
                if in_range(&right_pt, max_dim) {
                    test_pts.push(right_pt);
                }
            }
        }

        // Check this sensor's test points against the other sensors
        for pt in &test_pts {
            let mut is_beacon = true;
            for (&other_sensor, &other_beacon) in &loc_map {
                if other_sensor == sensor {
                    continue;
                }
                // If the point is in range of the sensor, this is not the beacon.
                let radius = distance(&other_sensor, &other_beacon);
                if distance(&other_sensor, &pt) <= radius {
                    is_beacon = false;
                    break;
                }
            }

            if is_beacon {
                beacon_position = *pt;
                tuning_frequency = (pt.0 as i64) * 4000000 + (pt.1 as i64);
                return (beacon_position, tuning_frequency);
            }
        }
    }

    // fallback return
    println!("WARNING: No beacon found. Returning default values.")
    (beacon_position, tuning_frequency)
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day15/test_input.txt" };
    let test_val: i32 = if args.len() > 2 { args[2].parse::<i32>().unwrap() } else { 10 };

    // Parse the file
    let data = fs::read_to_string(filename).unwrap();

    // PART 1
    let num_cannot_exist = search_row(&data, test_val);
    println!("\nPart 1: There are {} positions where beacon cannot exist\n", num_cannot_exist);

    // PART 2
    let (beacon_location, tuning_frequency) = search_beacons(&data, test_val);
    println!("\nPart 2: Beacon at {:?}, Frequency {}\n", beacon_location, tuning_frequency);
}

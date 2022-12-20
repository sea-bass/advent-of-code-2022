// Solution to Day 20 puzzle
// https://adventofcode.com/2022/day/20
//
// Example usage:
//   cargo run -r --bin day20 data/day20/test_input.txt

use std::env;
use std::fs;

fn mix(mut data: Vec<i64>, num_mixes: usize) -> Vec<i64> {
    let num_vals = data.len() as i64;
    let mut orig_indices: Vec<i64> = (0..num_vals).collect();

    for _ in 0..num_mixes {
        let mut vals_moved = 0;
        while vals_moved < num_vals {
            // Calculate the target index
            let cur_idx = orig_indices.iter().position(|&x| x == vals_moved)
                                      .unwrap() as i64;
            let mut target_idx = cur_idx + data[cur_idx as usize];
            while target_idx <= 0 {
                target_idx += num_vals - 1 ;
            }
            while target_idx >= num_vals {
                target_idx -= num_vals - 1;
            }
            // println!("Moving {} from index {} to {}",
            //     data[cur_idx as usize], cur_idx, target_idx);

            // Swap elements along the motion direction
            let dir = (target_idx - cur_idx).signum();
            let num_moves = (target_idx - cur_idx).abs();
            let mut orig = cur_idx;
            for _ in 0..num_moves {
                let new = orig + dir;
                data.swap(orig as usize, new as usize);
                orig_indices.swap(orig as usize, new as usize);
                orig = new;
            }

            vals_moved += 1;
            // println!("Step {}:\n{:?}", vals_moved, data);
        }
    }
    data
}

fn get_coordinates(data: &Vec<i64>) -> i64 {
    let zero_idx = data.iter().position(|&x| x == 0).unwrap();
    let n = data.len();
    let coordinates = data[(zero_idx + 1000) % n] +
                      data[(zero_idx + 2000) % n] +
                      data[(zero_idx + 3000) % n];
    coordinates
}


fn decrypt_type_1(data: Vec<i64>) -> i64 {
    // Mix the data once
    let data = mix(data, 1);

    // Compute the coordinates
    get_coordinates(&data)
}

fn decrypt_type_2(data: Vec<i64>) -> i64 {
    // Apply decryption key
    let data: Vec<i64> = data.iter().map(|x| x * 811589153).collect();

    // Mix the data a few times
    let data = mix(data, 10);
    
    // Compute the coordinates
    get_coordinates(&data)
}


fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day20/test_input.txt" };

    // Parse the input
    let data: Vec<i64> = fs::read_to_string(filename).unwrap()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // Part 1
    let coordinates = decrypt_type_1(data.clone());
    println!("\nPart 1: Coordinates = {}\n", coordinates);

    // Part 2
    let coordinates = decrypt_type_2(data.clone());
    println!("\nPart 2: Coordinates = {}\n", coordinates);
}

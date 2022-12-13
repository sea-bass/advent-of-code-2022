// Solution to Day 13 puzzle
// https://adventofcode.com/2022/day/13
//
// Example usage:
//   cargo run --bin day13 data/day13/test_input.txt

use std::env;
use std::fs;

fn compare(left: &str, right: &str) -> i32 {
    // println!("Comparing {} and {}", left, right);

    // Base case: If left or right are empty
    if left.is_empty() && right.is_empty() {
        return 0;
    } else if left.is_empty() && !right.is_empty() {
        return 1;
    } else if !left.is_empty() && right.is_empty() {
        return -1;
    }

    // Base case: If both values are integers
    if let (Ok(left_num), Ok(right_num)) = (left.parse::<u32>(), right.parse::<u32>()) {
        if left_num < right_num {
            return 1;
        } else if left_num > right_num {
            return -1;
        } else {
            return 0;
        }
    }

    // Now strip brackets and keep going recursively
    let left_has_brackets = left.chars().nth(0).unwrap() == '[' &&
                            left.chars().nth(left.len() - 1).unwrap() == ']';
    let right_has_brackets = right.chars().nth(0).unwrap() == '[' &&
                             right.chars().nth(right.len() - 1).unwrap() == ']';
    let mut new_left = left;
    if left_has_brackets {
        new_left = &left[1..left.len()-1];
    }
    let mut new_right = right;
    if right_has_brackets {
        new_right = &right[1..right.len()-1];
    }

    // At this point, no entities have brackets, so we'll loop through them.
    let mut index = 0;
    let left_parts = split(new_left);
    let right_parts = split(new_right);
    loop {
        let left_done = left_parts.len() < index + 1;
        let right_done = right_parts.len() < index + 1;
        if left_done && right_done {
            // println!("Both ran out of items");
            return 0;
        } else if left_done && !right_done {
            // println!("Left ran out of items");
            return 1;
        } else if !left_done && right_done {
            // println!("Right ran out of items");
            return -1;
        } else{
            let result = compare(left_parts[index], right_parts[index]);
            if result != 0 {
                return result;
            }
        }
        index += 1;
    }
}

fn split(input: &str) -> Vec<&str> {
    let mut output: Vec<&str> = Vec::new();
    let mut start_idx = 0;
    let mut opening_brackets = 0;

    for i in 0..input.len() {
        match input.chars().nth(i) {
            Some('[') => opening_brackets += 1,
            Some(']') => opening_brackets -= 1,
            Some(',') => {
                if opening_brackets == 0 {
                    output.push(&input[start_idx..i]);
                    start_idx = i+1;
                }
            },
            _ => {},
        }
    }
    output.push(&input[start_idx..]);
    output
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day13/test_input.txt" };

    let data = fs::read_to_string(filename).unwrap();
    let data_split = data.split("\n\n").collect::<Vec<&str>>();

    // Part 1: Find the sum of indices with pairs in the right order.
    let mut index = 1;
    let mut index_sum = 0;
    for elem in &data_split {
        let pair = elem.split("\n").collect::<Vec<&str>>();
        let result = compare(pair[0], pair[1]);
        // println!("Pair {} result {}\n", index, result);
        if result == 1 {
            index_sum += index;
        }
        index += 1;
    }
    println!("Part 1: Index sum = {}", index_sum);

    // Part 2: Add divider packets and then sort by comparison function
    let divider_packet_1 = "[[2]]";
    let divider_packet_2 = "[[6]]";
    let mut all_packets = Vec::from([divider_packet_1, divider_packet_2]);
    for line in data.lines() {
        if !line.is_empty() {
            all_packets.push(line);
        }
    }
    // Sort the packets using the comparison function and bubble sort (lol).
    // Implementation taken from https://www.hackertouch.com/bubble-sort-in-rust.html
    for i in 0..all_packets.len() {
        for j in 0..all_packets.len() - 1 - i {
            if compare(all_packets[j], all_packets[j + 1]) == -1 {
                all_packets.swap(j, j + 1);
            }
        }
    }
    // println!("Sorted packets:");
    // for packet in &all_packets {
    //     println!("{}", packet);
    // }

    // Find the indices for the divider packets
    let mut divider_idx_1: usize = 0;
    let mut divider_idx_2: usize = 0;
    let mut i = 1;
    for packet in &all_packets {
        if packet == &divider_packet_1 {
            divider_idx_1 = i;
        }
        if packet == &divider_packet_2 {
            divider_idx_2 = i;
        }
        i += 1;
    }

    let decoder_key = divider_idx_1 * divider_idx_2;
    println!("\nPart 2: Decoder key = {}\n", decoder_key);

}

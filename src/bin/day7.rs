// Solution to Day 7 puzzle
// https://adventofcode.com/2022/day/7
//
// Example usage:
//   cargo run --bin day7 test_input.txt

use std::collections::HashMap;
use std::cmp::min;
use std::env;
use std::fs;

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day7/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();

    // Parse the directories
    let mut folder_stack: Vec<String> = Vec::new();
    let mut folder_map = HashMap::<String, u32>::new();
    folder_map.insert("/".to_string(), 0);

    for line in data.lines() {

        let parts: Vec<&str> = line.split_whitespace().collect();
        let elem = parts[0];

        if elem == "$" {
            // Command case
            let cmd = parts[1];
            if cmd == "ls" {
                continue;
            } else if cmd == "cd" {
                let mut dir = parts[2].to_string();
                if dir == "/" {
                    dir = "".to_string();
                }
                if dir == ".." {
                    folder_stack.pop();
                } else {
                    let mut full_path = folder_stack.last()
                        .unwrap_or(&"".to_string())
                        .clone();
                    full_path.push_str("/");
                    full_path.push_str(&dir);
                    folder_stack.push(full_path);
                }
            }
        } else if elem == "dir" {
            // Directory case
            let mut full_path = folder_stack.last()
                        .unwrap_or(&"".to_string())
                        .clone();
            full_path.push_str("/");
            full_path.push_str(&parts[1].to_string());
            folder_map.entry(full_path).or_insert(0);
        } else {
            // File case
            let file_size = parts[0].parse::<u32>().unwrap();
            for dir in &folder_stack {
                if let Some(folder_size) = folder_map.get_mut(&dir.clone()) {
                    *folder_size += file_size;
                }
            }
        }
            
    }

    // Part 1: Print sizes
    const MAX_SIZE: u32 = 100000;
    let mut size_count = 0;
    for (name, &size) in &folder_map {
        if size <= MAX_SIZE {
            size_count += size;
        }
    }
    println!("\nPart 1:\nTotal size of files smaller than {}: {}\n",
        MAX_SIZE, size_count);


    // Part 2: Delete smallest directory
    const REQUIRED_SIZE: u32 = 70000000 - 30000000;
    let total_size = folder_map.get(&"/".to_string()).unwrap();
    println!("\nPart 2:\nTotal file size: {}", total_size);
    let mut smallest_dir_size = u32::MAX;
    for (name, &size) in &folder_map {
        if (total_size - size) <= REQUIRED_SIZE {
            smallest_dir_size = min(smallest_dir_size, size);
        }
    }
    println!("Deleting folder with size {} to get to {}\n",
        smallest_dir_size, total_size - smallest_dir_size);
}

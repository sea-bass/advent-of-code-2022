// Solution to Day 7 puzzle
// https://adventofcode.com/2022/day/7
//
// Example usage:
//   cargo run --bin day7 test_input.txt
//
// NOTE: This does not work yet!

use std::collections::HashMap;
use std::env;
use std::fs;

// File implementation
struct File {
    name: String,
    size: u32
}

impl File {
    fn print(&self) {
        println!("File {}, size {}", self.name, self.size);
    }
}

// Directory implementation
struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Directory>
}

impl Directory {
    fn new(name: String) -> Self {
        Self { 
            name, 
            files: Vec::<File>::new(),
            dirs: Vec::<Directory>::new()
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn print(&self) {
        println!("Directory {}, size {}", self.name, self.size());
        for dir in &self.dirs {
            dir.print();
        }
        for file in &self.files {
            file.print();
        }
    }

    fn size(&self) -> u32 {
        let mut size: u32 = 0;
        for dir in &self.dirs {
            size += dir.size();
        }
        for file in &self.files {
            size += file.size;
        }
        size
    }
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day7/test_input.txt" };

    // Read the file
    let data = fs::read_to_string(filename).unwrap();
    let chars = data.chars().collect::<Vec<char>>();

    // Parse the directories
    let mut folders = HashMap::new();
    let root_dir = Directory::new("/".to_string());
    folders.insert("/".to_string(), root_dir);
    let mut cur_dir = "/".to_string();
    let mut current_folder = folders.get_mut(&cur_dir).unwrap();
    for line in data.lines() {
        // println!("\nLine\n{}", line);

        let parts: Vec<&str> = line.split_whitespace().collect();

        let elem = parts[0];

        if elem == "$" {
            // Command case
            let cmd = parts[1];
            if cmd == "ls" {
                continue;
            } else if cmd == "cd" {
                cur_dir = parts[2].to_string();
            }
        } else if elem == "dir" {
            // Directory case
            let dirname = parts[1].to_string();
            let d = Directory::new(dirname.clone());
            folders.insert(dirname.clone(), d);

            parent_folder = folders.get_mut(&cur_dir).unwrap();

            // let new_folder = folders.get(&dirname.clone()).unwrap();
            parent_folder.dirs.push(d);

        } else {
            // File case
            // println!("Found file {} in {}", parts[1], cur_dir);
            let current_folder = folders.get_mut(&cur_dir).unwrap();
            current_folder.files.push(
                File{
                    name: parts[1].to_string(), 
                    size: parts[0].parse::<u32>().unwrap()
                }
            );
        }
            
    }

    // Print sizes
    const MAX_SIZE: u32 = 100000;
    let mut size_count = 0;
    for (name, dir) in &folders {
        let size = dir.size();
        println!("Folder {} has size {}", name, size);
        if size <= MAX_SIZE {
            println!("Adding to list");
            size_count += size;
        }
    }

    println!("\nTotal size: {}\n", size_count);

}

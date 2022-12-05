// Solution to Day 5 puzzle, Part 1
// https://adventofcode.com/2022/day/5
//
// Example usage:
//   cargo run --bin day5a test_input.txt

use std::env;
use std::fs;

struct Instruction {
    num: usize,
    from: usize,
    to: usize
}

struct BoxStacks {
    crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>
}

impl BoxStacks {
    fn new(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap();
        
        // Find where the newline is
        let mut pivot_idx: usize = 0;
        for (i, line) in data.lines().enumerate() {
            if line.is_empty() {
                pivot_idx = i;
                break;
            }
        }

        // Get the stacks
        let mut crates = Vec::new();
        let data = data.lines().collect::<Vec<&str>>();
        let num_crates = data[pivot_idx - 1].split_whitespace().last().unwrap()
            .parse::<usize>().unwrap();
        crates.resize(num_crates, Vec::new());

        for line in data[0..pivot_idx - 1].into_iter().rev() {
            for i in 0..num_crates {
                let idx = 4*i + 1;
                let chars = line.chars().collect::<Vec<char>>();
                if line.len() > idx && chars[idx] >= 'A' && chars[idx] <= 'Z' {
                    crates[i].push(chars[idx]);
                }
            }
        }

        // Get the instructions
        let mut instructions = Vec::new();
        for line in &data[pivot_idx+1..] {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            instructions.push(
                Instruction{num: split_line[1].parse::<usize>().unwrap(),
                            from: split_line[3].parse::<usize>().unwrap(),
                            to: split_line[5].parse::<usize>().unwrap()}
            );
        }

        Self { crates, instructions }
    }

    fn execute(&mut self) {
        self.print_crates();

        for inst in self.instructions.iter() {
            println!("Moving {} crates from {} to {}", inst.num, inst.from, inst.to);
            for _ in 0..inst.num {
                let item = self.crates[inst.from - 1].pop().unwrap();
                self.crates[inst.to - 1].push(item);
            }
            self.print_crates();
        }
    }

    fn get_last_elements(&self) {
        let mut print_str = String::new();
        for stack in &self.crates {
            print_str += &stack.last().unwrap().to_string();
        }
        println!("Final elements: {}", print_str);
    }

    fn print_crates(&self) {
        let num_crates = self.crates.len();
        let lengths = self.crates.iter()
            .map(|x| x.len()).collect::<Vec<usize>>();
        let max_len = lengths.iter().max().unwrap();

        let mut print_str = String::new();
        for i in (0..*max_len).rev() {
            let mut stack_str = String::new();
            for j in 0..num_crates {
                if self.crates[j].len() > i {
                    let item = self.crates[j][i].to_string();
                    stack_str.push_str(&item);
                } else {
                    stack_str.push_str(" ");
                }
            }
            print_str.push_str(&stack_str);
            print_str.push_str("\n");
        }

        println!("{}", print_str);
    }
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day5/test_input.txt" };

    // Run the code
    let mut box_stacks = BoxStacks::new(filename);
    box_stacks.execute();
    box_stacks.get_last_elements();
}

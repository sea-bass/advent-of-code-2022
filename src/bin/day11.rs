// Solution to Day 11 puzzle
// https://adventofcode.com/2022/day/11
//
// Example usage:
//   cargo run --bin day11 data/day11/test_input.txt

use std::env;
use std::fs;

extern crate evalexpr;
use evalexpr::*;

struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: String,
    test_divisible: i64,
    test_true_target: usize,
    test_false_target: usize,
    num_inspections: i64,
}

impl Monkey {
    fn new(data: &str) -> Self {
        let lines = data.split("\n").collect::<Vec<&str>>();

        // Line 0 is the ID
        let id_parts = lines[0].split(&[' ',':'][..]).collect::<Vec<&str>>();
        let id = id_parts[1].parse::<i64>().unwrap();

        // Line 1 is the starting items
        let items_parts = lines[1].split(": ").collect::<Vec<&str>>();
        let items = items_parts[1].split(", ")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();

        // Line 2 is the operation
        let operation_parts = lines[2].split("new = ").collect::<Vec<&str>>();
        let operation = operation_parts[1].to_string();

        // Lines 3, 4, and 5 are the test values
        let test_divisible = lines[3].split_whitespace().last().unwrap()
            .parse::<i64>().unwrap();
        let test_true_target = lines[4].split_whitespace().last().unwrap()
            .parse::<usize>().unwrap();
        let test_false_target = lines[5].split_whitespace().last().unwrap()
            .parse::<usize>().unwrap();
    
        Self { id, items, operation, test_divisible, test_true_target, test_false_target, num_inspections: 0 }
    }

    fn operate(&mut self, item_idx: usize) {
        let mut context = context_map!{"old" => self.items[item_idx]}.unwrap();
        self.items[item_idx] = 
            eval_int_with_context_mut(&self.operation[..], &mut context).unwrap();
    }

    fn print(&self) {
        println!("Monkey: {}, Items: {:?}, Inspections: {}", self.id, self.items, self.num_inspections);
    }
}


fn initialize_monkeys(filename: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let data = fs::read_to_string(filename).unwrap();
    let data_split = data.split("\n\n").collect::<Vec<&str>>();
    for monkey_text in &data_split {
        monkeys.push(Monkey::new(monkey_text));
    }
    monkeys
}

fn simulate_monkeys(monkeys: &mut Vec<Monkey>, rounds: usize, worry_divided: bool) {
    println!("=== INITIAL ===");
    for monkey in monkeys.iter() {
        monkey.print();
    }
    for round in 0..rounds {
        println!("=== ROUND {} ===", round + 1);
        // Inspections
        for m in 0..monkeys.len() {
            for i in (0..monkeys[m].items.len()).rev() {
                println!("  Monkey inspecting an item with worry level {}",
                    monkeys[m].items[i]);
                // Operate on worry levels
                monkeys[m].operate(i);

                // Boredom
                if worry_divided {
                    monkeys[m].items[i] /= 3;
                    println!("    Worry level is now {}", monkeys[m].items[i]);
                }

                // Update inspection count
                monkeys[m].num_inspections += 1;

                let item_val = monkeys[m].items[i];
                let is_divisible = (item_val % monkeys[m].test_divisible) == 0;
                let throw_target = match is_divisible {
                    true => monkeys[m].test_true_target,
                    false => monkeys[m].test_false_target,
                };

                // Do the actual throwing
                monkeys[throw_target].items.push(item_val);
                monkeys[m].items.remove(i);
                println!("    Worry level {} divisible by {}: {}\n    Throwing to monkey {}",
                    item_val, monkeys[m].test_divisible, is_divisible, throw_target);
            }
        }
        for monkey in monkeys.iter() {
            monkey.print();
        }
    }
}

fn get_monkey_business(monkeys: &mut Vec<Monkey>) -> i64 {
    let mut inspections_vec = monkeys.iter()
        .map(|x| x.num_inspections)
        .collect::<Vec<i64>>();
    inspections_vec.sort();
    inspections_vec.reverse();
    inspections_vec[0] * inspections_vec[1]
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day11/test_input.txt" };

    // Simulate Part 1
    let mut monkeys = initialize_monkeys(filename);
    simulate_monkeys(&mut monkeys, 20, true);
    println!("\nPart 1: Monkey business = {}", get_monkey_business(&mut monkeys));

    // Simulate Part 2
    // let mut monkeys = initialize_monkeys(filename);
    // simulate_monkeys(&mut monkeys, 10000, false);
    // println!("\nPart 2: Monkey business = {}", get_monkey_business(&mut monkeys));
}

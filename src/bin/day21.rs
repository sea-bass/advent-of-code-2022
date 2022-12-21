// Solution to Day 21 puzzle
// https://adventofcode.com/2022/day/21
//
// Example usage:
//   cargo run -r --bin day21 data/day21/test_input.txt

use std::collections::HashMap;
use std::env;
use std::fs;

extern crate itertools;
use itertools::Itertools;


type MonkeyData = HashMap<String, String>;

fn parse_data(filename: &str) -> MonkeyData {
    let raw_text = fs::read_to_string(filename).unwrap();
    let mut data = HashMap::new();
    for line in raw_text.lines() {
        let (key, val) = line.split(": ").next_tuple().unwrap();
        data.insert(key.to_string(), val.to_string());
    }
    data
}

fn eval_expression(data: &MonkeyData, monkey_name: &str) -> i64 {
    let expr_str = &data[monkey_name];

    if let Ok(num) = expr_str.parse::<i64>() {
        // Base Case: If the expression parses to a number, return the number
        return num;
    } else {
        // Recursive Case: Split the string and evaluate each sub-portion
        let (left, oper, right) = expr_str.split_whitespace().next_tuple().unwrap();
        return match oper {
            "+" => eval_expression(&data, left) + eval_expression(&data, right),
            "-" => eval_expression(&data, left) - eval_expression(&data, right),
            "*" => eval_expression(&data, left) * eval_expression(&data, right),
            "/" => eval_expression(&data, left) / eval_expression(&data, right),
            _ => {
                println!("Warning: Invalid operation {}. Returning zero.", oper);
                0
            }
        }
    }
}


fn find_human_answer(data: &MonkeyData) -> i64 {
    // First, parse the "root" expression.
    let root_str = &data[&String::from("root")];
    let (root_left, _, root_right) = root_str.split_whitespace().next_tuple().unwrap();

    // Figure out whether the left or right side contains "humn" by evaluating twice.
    let left_control = eval_expression(&data, root_left);
    let right_control = eval_expression(&data, root_right);

    let mut new_data = data.clone();
    let humn_val = data[&String::from("humn")].parse::<i64>().unwrap() * 100;  // Some random mutation
    *new_data.get_mut(&String::from("humn")).unwrap() = humn_val.to_string();
    let left_test = eval_expression(&new_data, root_left);
    let right_test = eval_expression(&new_data, root_right);
    if left_test != left_control {
        println!("humn is on the left side of root.");
        return tweak_human_answer(&data, &data[root_left], right_test);
    } else if right_test != right_control {
        println!("humn is on the right side of root.");
        return tweak_human_answer(&data, &data[root_right], left_test);
    }
    
    println!("Warning: Neither left nor right side modified. Returning zero.");
    0
}

fn tweak_human_answer(data: &MonkeyData, expr: &str, answer: i64) -> i64 {
    // println!("Tweaking {} to be equal to {}", expr, answer);
    let (left, oper, right) = expr.split_whitespace().next_tuple().unwrap();
    let left_control = eval_expression(&data, left);
    let right_control = eval_expression(&data, right);

    // Recursive case: Figure out which side of the tree contains humn
    let mut new_data = data.clone();
    let humn_val = data[&String::from("humn")].parse::<i64>().unwrap() * 100;  // Some random mutation
    *new_data.get_mut(&String::from("humn")).unwrap() = humn_val.to_string();
    let left_test = eval_expression(&new_data, left);
    let right_test = eval_expression(&new_data, right);
    if left_test != left_control {
        // println!("humn is on the left side.");
        let new_answer = match oper {
            "+" => answer - right_test,
            "-" => answer + right_test,
            "*" => answer / right_test,
            "/" => answer * right_test,
            _ => {
                println!("Warning: Invalid operation {}. Returning zero.", oper);
                0
            }
        };
        if left == "humn" {
            // println!("humn should be equal to {}", new_answer);
            return new_answer;
        }
        return tweak_human_answer(&data, &data[left], new_answer);
    } else if right_test != right_control {
        // println!("humn is on the right side");
        let new_answer = match oper {
            "+" => answer - left_test,
            "-" => left_test - answer,
            "*" => answer / left_test,
            "/" => left_test / answer,
            _ => {
                println!("Warning: Invalid operation {}. Returning zero.", oper);
                0
            }
        };
        if right == "humn" {
            // println!("humn should be equal to {}", new_answer);
            return new_answer;
        }
        return tweak_human_answer(&data, &data[right], new_answer);
    }

    println!("Warning: Neither left nor right side modified. Returning zero.");
    0
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day21/test_input.txt" };

    // Parse the input
    let data = parse_data(filename);

    // Part 1
    let expr = eval_expression(&data, "root");
    println!("\nPart 1: Answer = {}\n", expr);

    // Part 2
    let answer = find_human_answer(&data);
    println!("\nPart 2: Answer = {}\n", answer);
}

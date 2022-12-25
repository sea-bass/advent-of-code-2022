// Solution to Day 25 puzzle
// https://adventofcode.com/2022/day/25
//
// Example usage:
//   cargo run --bin day25 data/day25/test_input.txt

use std::env;
use std::fs;

fn snafu_to_dec(snafu: &str) -> i64 {
    let len = snafu.len();
    let mut dec_val: i64 = 0;
    for (pos, ch) in snafu.chars().enumerate() {
        let digit: i64 = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
             _  => {
                println!("Bad number, should not happen.");
                0
             }
        };
        let exponent = (len - 1 - pos) as u32;
        dec_val += i64::pow(5, exponent) * digit; 
    } 
    dec_val
}

fn dec_to_snafu(dec: i64) -> String {
    let mut s = String::from("");
    let mut quotient = dec;
    while quotient > 0 {
        let remainder = (quotient + 2) % 5;
        quotient = (quotient + 2) / 5;
        s.push_str(match remainder {
            4 => "2",
            3 => "1",
            2 => "0",
            1 => "-",
            0 => "=",
            _ => "X",
        });
    }    
    s.chars().rev().collect()
}

fn get_fuel_cost(filename: &str) -> (i64, String) {
    let data = fs::read_to_string(filename).unwrap();
    let dec_val: i64 = data.lines()
                            .map(|line| snafu_to_dec(line))
                            .sum();
    (dec_val, dec_to_snafu(dec_val))
}

fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day25/test_input.txt" };

    let (cost_dec, cost_snafu) = get_fuel_cost(&filename);
    println!("\nPart 1: Fuel cost = {} decimal / {} SNAFU\n", cost_dec, cost_snafu);
}

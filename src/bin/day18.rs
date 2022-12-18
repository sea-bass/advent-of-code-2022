// Solution to Day 18 puzzle
// https://adventofcode.com/2022/day/18
//
// Example usage:
//   cargo run --bin day18 data/day18/test_input.txt

use std::collections::HashSet;
use std::env;
use std::fs;

extern crate itertools;
use itertools::Itertools;

const GRID_SIZE: usize = 20;  // From visually inspecting data

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

// Returns all adjacent points to a cube
fn get_adjacent_points(pt: &Point) -> Vec<Point> {
    vec![
        Point{x: pt.x + 1, y: pt.y, z: pt.z},
        Point{x: pt.x - 1, y: pt.y, z: pt.z},
        Point{x: pt.x, y: pt.y + 1, z: pt.z},
        Point{x: pt.x, y: pt.y - 1, z: pt.z},
        Point{x: pt.x, y: pt.y, z: pt.z + 1},
        Point{x: pt.x, y: pt.y, z: pt.z - 1},
    ]  
}

// Expansion algorithm for Part 2
fn is_face_exposed(lava_cubes_hash: &HashSet<Point>, pt: &Point) -> bool {
    let mut cube_queue = Vec::new();
    let mut visited_cubes = HashSet::new();
    cube_queue.push((pt.x, pt.y, pt.z));

    while !cube_queue.is_empty() {
        // Check the current point
        let cur_pt_tup = cube_queue.pop().unwrap();
        let cur_pt = Point{x: cur_pt_tup.0, y: cur_pt_tup.1, z: cur_pt_tup.2};
        visited_cubes.insert(cur_pt);
        if lava_cubes_hash.contains(&cur_pt) {
            continue;
        }
        let pt_exposed = cur_pt.x <= 0 || cur_pt.x >= (GRID_SIZE as i32) ||
                         cur_pt.y <= 0 || cur_pt.y >= (GRID_SIZE as i32) ||
                         cur_pt.z <= 0 || cur_pt.z >= (GRID_SIZE as i32);
        if pt_exposed {
            return true;
        }

        // Now expand all directions
        for p in get_adjacent_points(&cur_pt).iter() {
            if !visited_cubes.contains(&p) {
                cube_queue.push((p.x, p.y, p.z));
            }
        }
    }

    // If the queue was emptied before finding an exposed face, it's trapped air!
    false
}


fn calc_surface_area(lava_cubes: &HashSet<Point>,
                     consider_trapped_air: bool) -> u32 {
    let mut total_surface_area = 0;
    for cube in lava_cubes {
        // Check all 6 cube faces
        for pt in get_adjacent_points(&cube).iter() {
            let add_face = match consider_trapped_air {
                true => is_face_exposed(&lava_cubes, &pt),
                false => !lava_cubes.contains(&pt),
            };
            if add_face {
                total_surface_area += 1;
            }
        }
    }
    total_surface_area
}


fn main() {
    // Get the filename from the command line, else fall back to default
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "data/day18/test_input.txt" };
    
    // Parse input
    let data = fs::read_to_string(filename).unwrap();
    let mut lava_cubes = HashSet::new();
    for line in data.lines() {
        let (x, y, z) = line.split(",")
                             .map(|x| x.parse::<i32>().unwrap())
                             .next_tuple()
                             .unwrap();
        lava_cubes.insert(Point {x, y, z});
    }

    // Part 1
    let surface_area = calc_surface_area(&lava_cubes, false);
    println!("Part 1: Total surface area = {}", surface_area);

    // Part 2
    let surface_area_with_air = calc_surface_area(&lava_cubes, true);
    println!("Part 2: Total surface area = {}", surface_area_with_air);
}

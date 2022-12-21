// Getting used to the ndarray crate
//
// To run this,
//   cargo run --bin hello_ndarray

extern crate ndarray;
use ndarray::{Array2, s};

fn main() {
    // Create a 2D array
    let mut arr = Array2::<i32>::zeros((10, 10));

    // Modify single elements
    arr[[1, 3]] = 5;
    arr[[2, 2]] = 6;

    // Modify columns
    for elem in arr.slice_mut(s![.., 6..=7]) {
        *elem = 2;
    }

    // Print the results
    println!("My first ndarray:\n{:?}", arr);
}

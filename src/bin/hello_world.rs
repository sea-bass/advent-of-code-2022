// Your regular hello world, basically copied from
// https://doc.rust-lang.org/stable/rust-by-example/hello.html
//
// To run this,
//   rustc hello_world.rs
//   ./hello_world

// Helper function that prints with a prefix
fn do_print(message: &str) {
    println!("Hello from Rust: {}", message);
}

// Main function
fn main() {
    do_print("First message");
    do_print("Second message");
}

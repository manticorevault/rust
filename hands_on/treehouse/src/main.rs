// Import the input from std library
use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");

    // Store the user's name as a mut variable
    let mut your_name = String::new();

    // Grabs the user input
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    println!("Hello, {}", your_name);
}

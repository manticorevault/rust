// Import the IO input/output library from the
// standard library
use std::io;

// Start the main function
fn main() {
    // Print the instructions
    println!("Guess the correct number!");
    println!("Input your guess: ");

    // Store user input in a mutable variable as a string
    let mut guess = String::new();

    // Use the stdin function to receive user's input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Output the user's guess using placeholders
    println!("You guessed: {}", guess)

}
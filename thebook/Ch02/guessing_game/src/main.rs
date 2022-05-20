// Import the IO input/output library from the
// standard library
use std::io;
// Import the comparing library
use std::cmp::Ordering;
// Import the rand library
use rand::Rng;

// Start the main function
fn main() {
    // Print the instructions
    println!("Guess the correct number between 1 and 100!");

    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Create a loop to run the guesses' input
    loop {
        println!("Input your guess: ");
            // Store user input in a mutable variable as a string 
        let mut guess = String::new();

        // Use the stdin function to receive user's input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user input from string to an unsigned integer
        let guess: u32 = match guess
                            .trim()
                            .parse() {
                                    Ok(num) => num,
                                    Err(_) => continue,
                                };

        // Output the user's guess using placeholders
        println!("You guessed: {}", guess);

        // Match the guess with the secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, you won!");
                break;
            }
        }
    }
}
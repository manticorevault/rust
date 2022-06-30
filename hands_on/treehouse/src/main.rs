// Import the input from std library
use std::io::stdin;

// Adding the visitor struct
struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!(
            "{}", self.greeting
        )
    }
}

fn name_catcher() -> String {
    // Store the user's name as a mut variable
    let mut your_name = String::new();

    // Grabs the user input
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    
    // Returns the name
    your_name
        .trim()
        .to_lowercase()
}


fn main() {
    println!("Hello, what's your name?");
    let name = name_catcher();
    let visitor_list = [
        Visitor::new("artur", "Ola, Artur!"),
        Visitor::new("masha", "Ola, Masha! Tu ta gostosa!"),
        Visitor::new("josephine", "Ola, Josephine!"),
    ];

    let known_visitors = visitor_list
                            .iter()
                            .find(| visitor|
                                    visitor.name == name);

    match known_visitors {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Sorry, your name is not on the list")
    }
}

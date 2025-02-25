use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100:");

    let random_number = rand::rng().random_range(1..=10).to_string();
    println!("Random number: {}", random_number);

    let mut guessed_correctly = false;
    
    while !guessed_correctly {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again");
                continue;
            }
        };

        println!("Your guess: {}", input);

        match input.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too low try again"),
            std::cmp::Ordering::Greater => println!("Too high try again"),
            std::cmp::Ordering::Equal => {
                println!("yippeee doneee!");
                guessed_correctly = true;
            }
        }
    }
}

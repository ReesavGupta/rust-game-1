use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100:");

    let random_number = rand::rng().random_range(1..=10);
    println!("(For testing) Random number: {}", random_number);

    let mut guessed_correctly = false;

    while !guessed_correctly {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        println!("Your guess: {}", input);

        if random_number.to_string() == input {
            println!("You guessed the correct number!");
            guessed_correctly = true;
        } else {
            println!("Wrong guess! Try again.");
        }
    }
}

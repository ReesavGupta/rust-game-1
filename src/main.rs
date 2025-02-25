use std::io;
fn main() {
    println!("Guess a number: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("error: {error}");
        }
    }
}

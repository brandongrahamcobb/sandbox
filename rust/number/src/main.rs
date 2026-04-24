use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret: u8 = rand::rng().random_range(1..100); // cargo doc --open
    println!("Please input your guess.");
    let mut guess: String = String::new();
    let mut status: bool = false;
    let mut i: u8 = 0;
    while !status && i < 10 {
        guess.clear(); // Otherwise read_line appends to the buffer.
        io::stdin()
            .read_line(&mut guess) // Modifies guess.
            .expect("Failed to read line");
        print!("You guessed: {}", guess);
        let guess: u8 = match guess.trim().parse() {
            // Match compares a value (Result) to options
            // (Ok, Err). (_) is a catchall.
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Guess {}. Too small!", i + 1);
            }
            Ordering::Greater => {
                println!("Guess {}. Too big!", i + 1);
            }
            Ordering::Equal => {
                println!("Guess {}. Correct!", i + 1);
                status = true;
            }
        }
        i += 1;
    }
}

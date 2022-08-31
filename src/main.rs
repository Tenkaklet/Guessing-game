use rand::Rng; // random number
use std::cmp::Ordering; // Comparison crate
use std::io; // standard lib...

fn main() {
    
    println!("Guess the number!");

    // Random Number generated.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new(); // New mutable variable.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // read the input

        let guess: u32 = match guess.trim().parse() {
            // work with the input if it is converted to small number.
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // User input

        match guess.cmp(&secret_number) {
            // Comparison of input and Random Number.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! ");
                break;
            }
        }
    }
}

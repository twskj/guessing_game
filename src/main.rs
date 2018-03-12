extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

/**
 * A guessing game that illustrate how Rust syntax and features are used
 * Note worthy here are:
 * 
 * 1. There is a concept of macro like in C --> println!
 * 2. Error handling is done using match to ensure all cases are handled
 * 3. Many function return enum and will be processed using match
 * 4. Unicode friendly 👍
 * 5. Variable shadowing --> ln. 30 overwrite ln. 24 in guess declaration
 */

fn main() {
    let secret_number = rand::thread_rng().gen_range(0, 100);

    println!("-=Guess the Number=-");
    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num
            , Err(_) => {
                println!("'{}' is not a number", guess.trim());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 👍");
                std::process::exit(0);
            }
        }
    }
}

use std::cmp::Ordering;
use std::io;

use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess...");

        let mut your_guess = String::new();
        io::stdin()
            .read_line(&mut your_guess)
            .expect("Failed to read line...");

        let guess: u32 = match your_guess
            .trim()
            .parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter integer between 0 to 100 only.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {
                println!("{}","Correct guess, you are a winner !".green().bold());
                break;
            }
            Ordering::Greater => println!("{}","Too big".yellow())
        }
    }
}

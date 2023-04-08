use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess...");

        let mut your_guess = String::new();
        io::stdin()
            .read_line(&mut your_guess)
            .expect("Failed to read line...");

        let guess: u32 = your_guess
            .trim()
            .parse()
            .expect("Please type an integer only.");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct guess, you are a winner !");
                break;
            }
            Ordering::Greater => println!("Too big")
        }
    }
}

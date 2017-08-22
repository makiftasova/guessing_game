extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the Number");

    let secret = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;

    loop {

        print!("Enter your guess > ");
        io::stdout().flush().unwrap();

        // create a mutable String variable to store guess
        let mut guess = String::new();

        // expect -> set error message for panic!
        io::stdin().read_line(&mut guess).expect(
            "ERROR: Failed to red line!",
        );

        // read uint64 from String, and handle format errors
        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an unsigned number!");
                continue;
            }
        };

        tries += 1;

        if guess > 100 || guess < 1 {
            println!("Please enter a numbe between 0 and 100");
            continue;
        }

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Found the Number after {tries} tries", tries = tries);
                break;
            }
        };
    }
}

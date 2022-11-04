use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Welcome in the Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("What is your guess?");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Cannot read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        println!("Your guess was {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


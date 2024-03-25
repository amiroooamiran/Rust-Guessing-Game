extern crate rand;

use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Welcome to Guess Game");

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("Pleas Enter your Guess!");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guss is {}", guess);

        println!("Pleas enter your guess!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("the is less"),
            Ordering::Equal => println!("Win"),
            Ordering::Greater => {
                println!("This is Grate");
                break;
            }, 
        }
    }
}

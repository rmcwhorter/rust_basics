extern crate rand;
use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("SOF");
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    let mut rng = thread_rng();
    let secret_number: f64 = rng.gen_range(1,101) as f64;
    
    println!("{}", secret_number);


    println!("EOF");
}
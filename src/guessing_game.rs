use std::io;

use rand::Rng;

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=2);

    println!("Please input your guess (b/w 1-2):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Secret number {secret_number}");
    println!("You guessed: {guess}");
}
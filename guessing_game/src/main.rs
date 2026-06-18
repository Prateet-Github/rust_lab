use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    let mut rng = rand::thread_rng(); // create a random number generator

    let secret_number = rng.gen_range(1..=100); // generate within this range

    println!("The secret number is: {secret_number}");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

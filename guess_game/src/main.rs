use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number game");

    let random_number = rand::thread_rng().gen_range(1..101);

    // println!("the random number is: {}", random_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("correct");
                break;
            }
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("lesser"),
        }
    }
}

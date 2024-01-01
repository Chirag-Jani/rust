// importing library for input and output
use rand::Rng;
use std::{cmp::Ordering, io};

// entry point
fn main() {
    println!("Guess the Number!!!");
    println!("Enter your Guess: ");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // println!("Generated number is : {random_number}");

        // defining variable
        // mut keyword coz by default variables are immutable
        let mut guess = String::new();

        // taking input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // printing the input

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Go Higher"),
            Ordering::Greater => println!("Go lesser"),
            Ordering::Equal => {
                println!("Correct!! You won");
                break;
            }
        }
    }
}

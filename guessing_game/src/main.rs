use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game");

    let number = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();

    println!("Try your luck enter a number in range (1-100)");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You are enterred: {}", guess);
}

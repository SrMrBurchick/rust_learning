use std::io;

fn main() {
    println!("Guessing game");

    let mut guess = String::new();

    println!("Try your luck enter a number");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You are enterred: {}", guess);
}

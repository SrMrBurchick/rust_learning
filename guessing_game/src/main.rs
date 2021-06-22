use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game");

    let number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("Try your luck enter a number in range (1-100)");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Not a number");

        match guess.cmp(&number) {
            Ordering::Less => println!("You lose, litle cokcsuccer"),
            Ordering::Greater => println!("You lose, litle cokcsuccer"),
            Ordering::Equal => {
                println!("Hm.. ok, you win");
                break;
            }
        }

    }
}

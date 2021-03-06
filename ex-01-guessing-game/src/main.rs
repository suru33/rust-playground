use std::cmp::Ordering;
use std::io;
use std::process::exit;

use rand::Rng;

fn main() {
    println!("The guessing game");
    println!("Enter number");

    let secret: u32 = rand::thread_rng().gen_range(1..101);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Not a number");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!!!");
            exit(0);
        }
    }

    println!("Secret number: {}", secret);
}

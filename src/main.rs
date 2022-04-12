use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::{thread, time};

fn main() {
    println!("Welcome to Crispr guessing game!");
    println!("Guess the number!");

    let num1: u32 = rand::thread_rng().gen_range(1..=50);
    let num2: u32 = rand::thread_rng().gen_range(50..=100);

    let secret_number = rand::thread_rng().gen_range(num1..=num2);

    println!("The number is between {} and {}", num1, num2);

    loop {
        println!("please input your guess:");

        //println!("The secret number is: {}", secret_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small :c");
            }

            Ordering::Greater => {
                println!("Too big :c");
            }

            Ordering::Equal => {
                println!("You Win :D");
                thread::sleep(time::Duration::from_secs(3));
                break;
            }
        }
    }
}

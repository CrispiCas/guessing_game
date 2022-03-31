use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Crispr guessing game!");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("please input your guess or quit with 'quit':");


        //println!("The secret number is: {}", secret_number);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error: failed to read line");

        let guess : u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please Input a number");
                continue
            },
        };

        println!("You guessed: {}", guess);



        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small :c");
            },

            Ordering::Greater => {
                println!("Too big :c");
            },

            Ordering::Equal => {
                println!("You Win :D");
                break;
            }
        }
    }
}

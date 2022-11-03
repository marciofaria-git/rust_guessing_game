use rand::Rng;
use std::cmp::Ordering; // declaration for compare the number to guess
use std::io; // input/output library - the library comes from the standard library - std // Random Number library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // to make a variable mutable, just add "mut" before the variable name

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Just numbers");
                continue;
            }
        };

        println!("You guessed:{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}

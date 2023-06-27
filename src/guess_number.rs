use std::io::{self, Write};
use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;

pub fn guess_the_number() {
    println!("-------------------------------------------------------------");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The Secret Number is {}", secret_number);

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid number!".red());
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            },
        };
    }
}
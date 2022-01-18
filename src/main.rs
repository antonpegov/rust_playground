mod config;
mod models;
mod routes;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

use crate::config::print_config as log_config;
use crate::routes::user_route;

fn main() {
    // mods playground
    log_config();
    user_route::print_user_route();
    routes::index_route::print_index_route();

    // game
    loop {
        println!("Guess the number from 1 to 10!");
        println!("Please input your guess.");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..11);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("I guessed: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                break;
            }
        }
    }
}

//standard(std) input output(io) library
// By default, Rust has a set of items defined in the standard library that it
//brings into the scope of every program. This set is called the prelude, and
//you can see everything in it at
//*https:doc.rust-lang.org/std/prelude/index.xhtml.
//If a type you want to use isn’t in the prelude, you have to bring that type
//into scope explicitly with a use statement.
//use rand as dependency in cargo.toml to use
use rand::Rng;
use std::cmp::Ordering;
use std::io;

//cargo doc --open
//creates local documentation for the cargo and opens in browser

pub fn guess() {
    println!("Guess The Number For $10million !");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //infinte loop
    loop {
        println!("input your guess from 1 to 100:");
        //using rand crate to generate random number

        //variable to store the user input
        let mut guess = String::new();
        //:: defines it is part of the function/file
        //readline takes imput and append it into string
        // .expect is a "Result" value and send warning if not used
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //Guess is already defined but u can redefine it shadowing the previous one.This technique is called shadowing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is catch all value
            Err(_) => continue,
        };

        //converts string to another type

        println!("you guessed:{guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Small pp"),
            Ordering::Greater => println!("Too big pp"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
    println!("The secret number was indeed:{secret_num}")
}

// Lesson 1: Basic User Input Handling

use std::io; // Import the standard input/output module

fn main()
{
    // create mutable variable to store user input
    let mut input = String::new();

    println!("Enter your name: ");

    // Read user input
    io::stdin().read_line(&mut input).expect("Failed input");

    println!("Merhaba, {}", input);

}
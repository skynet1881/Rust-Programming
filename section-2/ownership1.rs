// Rust Memory Safety and Ownership Basics
// Ownership is one of the most unique features of Rust. It is a way to manage memory in Rust. 
// Ownership rules are checked at compile time. Ownership rules are:
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
//

// Ownership basics in Rust

fn main()
{
    let s1 = String::from("hello"); // s1 is the owner of the string "hello"
    let s2 = s1; // s2 is the owner of the string "hello"

    println!("{}", s2); // s2 is the owner of the string "hello"
}
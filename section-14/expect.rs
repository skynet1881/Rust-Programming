// `expect()` is a shortcut to handle errors. It either gives the value or stops the program with a message.
use std::fs::File;

fn main()
{
    let result = File::open("example2.txt").expect("Failed to open file");

    println!("File opened successfully: {:?}", result);
}
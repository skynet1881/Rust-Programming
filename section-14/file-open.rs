use std::fs::File;

//std → The Rust standard library.
//fs → The filesystem module inside the standard library, which provides utilities for working with the file system (reading, writing, opening, etc.).
//File → A struct inside std::fs that represents a file and provides methods for file operations

fn main()
{
    // Try open file
    let result = File::open("example.txt");

    match result
    {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open file: {:?}", error),
    }
}
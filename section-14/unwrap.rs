// Using `unwrap()` for Quick Error Handling
// `unwrap()` is similar to `expect()`, but it does not allow custom messages.

use std::fs::File;

fn main()
{
    let result = File::open("example2.txt").unwrap();

    println!("File opened successfully: {:?}", result);
}
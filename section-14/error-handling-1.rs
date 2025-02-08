// Lesson 1: What is Error Handling?
// Rust has two types of errors:
// 1. Recoverable errors: These can be handled, like file not found. Rust uses `Result<T, E>` for these.
// 2. Unrecoverable errors: These cause the program to stop immediately using `panic!()`.

fn main()
{
    // Example of unrecoverable error
    let number = 0;
    if number == 0
    {
        panic!("Something went wrong, stop execution");
    }

    println!("Set voltages to 5V");
}
// Shadowing in Rust
// Shadowing allows reusing variable names while changing their type or value.
// Unlike `mut`, it creates a new variable rather than modifying the existing one.

fn main()
{
    let x = 5; // immutable variable
    println!("The value of x is: {}", x);

    //x = 6; // error: can not assign immutable
    // shadowing
    let x = 6;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    // Shadow x to different type
    let x = "Hello, World!";
    println!("The value of x is: {}", x);

    // Shadowing is different from mutability
    // mut, you can change the value but not the type
    let mut y = 5;
    y = 6;
    println!("The value of y is: {}", y);

    //y = "Hello World"; // type is integer, can not assign string
}
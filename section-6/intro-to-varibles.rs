// Intro to variables in Rust

fn main()
{
    // Default variables are immutable
    let x = 10; // x is immutable, I can't change the value of x
    println!("Value of x: {}", x);

    let y: i32 = 21; // y is immutable, I can't change the value of y
    println!("Value of y: {}", y);
    y = 22; // This will give an error because y is immutable
}
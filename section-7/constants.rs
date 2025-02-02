// Introduttion to constants in Rust

const PI: f64 = 3.14159;
const MAX_USER: u32 = 100;

fn main()
{
    println!("PI: {}", PI);
    println!("MAX_USER: {}", MAX_USER);

    // Constants are immutable
    // PI = 3.14; // Error: cannot assign twice to immutable variable

    // Immutable variables
    let _x = 10;
}
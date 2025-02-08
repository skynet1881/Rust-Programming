// Lesson 1: Primitive data types in Rust

// What are primitive data types in Rust?
// - Primitive data types are the most basic data types available within the Rust language.
// - They are built into the language and are directly supported by the compiler.
// - They are the simplest data types available in Rust.
// What are they? 
// - Integer types: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
// - Floating-point types: f32, f64
// - Boolean type: bool
// - Character type: char

fn main()
{
    // Integer types: i8, i16, i32, i64, i128 (signed)
    //                u8, u16, u32, u64, u128 (unsigned)
    let int_signed: i32 = -10; // signed integer 32 bit
    let int_unsigned: u32 = 10; // unsigned integer 32 bit
    let int_signed_8: i8 = -1; // signed integer 8 bit

    // Floating point types: f32, f64
    let float_32: f32 = 3.14; // 32 bit floating point
    let float_64: f64 = 1.81; // 64 bit floating point

    // Boolean type: bool
    let is_true: bool = true;
    let is_false: bool = false;

    // Character type (Single Quotes): char
    let character: char = 'C';

    // Print the values
    println!("Integer signed: {}", int_signed);
    println!("Integer unsigned: {}", int_unsigned);
    println!("Integer signed 8: {}", int_signed_8);
    println!("Float 32: {}", float_32);
    println!("Float 64: {}", float_64);
    println!("Boolean true: {}", is_true);
    println!("Boolean false: {}", is_false);
    println!("Character: {}", character);
}

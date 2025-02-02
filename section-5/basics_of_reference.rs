// Basics of reference in Rust
// Rust has a feature called references which allow you to refer to some value without taking ownership of it.

// Primitive types like integers, floats are copied directly. But for complex types like vectors, strings, etc. the ownership is transferred to the new variable.
// so you need borrowing for non - primitive types.
fn main()
{
    let x : i32 = 10; // primitive type
    let y = x; // copied directly

    println!("x = {}, y = {}", x, y); // both can be used, ownership not transferred

    let s1 = String::from("Merhaba"); // complex type
    let s2 = s1; // ownership transferred to s2

    //println!("s1 = {}, s2 = {}", s1, s2); // error: value borrowed here after move, I can not use s1 anymore

    let s_ref = &s2; // reference to s2, borrowing
    println!("s2 = {}, s_ref = {}", s2, s_ref); // both can be used
}
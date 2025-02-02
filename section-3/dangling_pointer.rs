// Dangling Reference Handling in Rust

// Rust ensures that dangling references are not possible.
// What is dangling reference?
// A dangling reference is a pointer that references a location in memory that may have been given to someone else, freed or deallocated.

fn dangling_reference() -> &String
{
    let s = String::from("Merhaba Dunya");
    &s // s will be deallocated when the function ends
}

fn valid_return() -> String
{
    let s = String::from("Merhaba Dunya");
    s
}

fn main()
{
    let s = valid_return();
    println!("The value of s is: {}", s);
}

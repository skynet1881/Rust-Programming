// Move semantic and borrowing

fn main()
{
    let s = String::from("Rust");

    // Move: ownership will be transfer
    let _s2 = s; // s is invalid

    // println!("s: {}", s); // Error: value borrowed here after move

    let s3 = String::from("Ownership");

    // Borrowing: ownership will not be transfer
    // immutable, read-only
    let len = calculate_length(&s3);
    println!("Length of '{}' is {}", s3, len);
}

fn calculate_length(s: &String) -> usize
{
    s.len() // only reading, only return length
}
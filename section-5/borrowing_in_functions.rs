// Borrowing in functions in Rust

fn print_length(s: &String)
{
    println!("Length of the string is: {}", s.len());
}

fn modify_string(s: &mut String)
{
    s.push_str("Turkiye!");
}

fn main()
{
    let s = String::from("Merhaba Turkiye");
    print_length(&s); // immutable borrowing
    println!("{}", s); // s is still valid

    // mutable borrowing
    let mut s = String::from("Merhaba, ");
    modify_string(&mut s); // mutable borrowing
    println!("{}", s); // s is still valid
}
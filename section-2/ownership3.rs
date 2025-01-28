// Returning ownership of a value from a function

fn gives_ownership() -> String
{
    let s = String::from("Hello");
    s // s is the owner of the string "Hello"
}

fn main()
{
    let s1 = gives_ownership(); // s1 is the owner of the string "Hello"

    println!("{}", s1); // s1 is the owner of the string "Hello"
}
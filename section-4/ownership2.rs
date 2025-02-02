// Ownership in functions

fn takes_ownership(s: String)
{
    println!("{}", s); // s is the owner of the string "Merhaba"
}

fn main()
{
    let s1 = String::from("Merhaba"); // s1 is the owner of the string "Merhaba"

    takes_ownership(s1); // Owner of the string "Merhaba" is transferred to the function
    // s1 no longer valid here
    println!("{}", s1); // This will give an error because s1 is no longer the owner of the string "Merhaba"
}
// Tekrar 1: Ownerhip Rules
// The Ownership Rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

/*
Scope:
{
    let s = String::from("hello");
    // s is valid from this point forward

    // do stuff with s
}
*/

fn main()
{
    // Rule 1 - Each value has one owner
    let s1 = String::from("Merhaba"); // s1 owns Merhaba string

    //let _s2 = s1; // s2 owns Merhaba string, s1 is not invalid
    // Rule 2 - One owner at a time
    //println!("{} {}", _s2, s1);

    let s2 = s1.clone(); // s2 owns Merhaba, s1 owns Merhaba
    println!("{} {}", s1, s2);

    // Rule 3 - the value will be dropped, when owner goes out of scope
    {
        let s3 = String::from("Dünya"); // s3 owns Dünya
        println!("{}", s3);
    } // s3 is out of scope, will be dropped
    //println!("{}", s3);
    println!("{} {}", s1, s2);
} // s1 and s2 are out of scope, will be dropped here

// Borrowing and Ownership Rules in Rust

fn main()
{
    let s = String::from("Merhaba"); // complex data type
    //let _ref1 = s; // Ownership transfered to ref1
    //println!("{}", s); // Error: value borrowed here after move

    // Immutable reference
    let _ref1 = &s; // ownership not transfered
    let _ref2 = &s;
    println!("{} and {}", _ref1, _ref2);
    println!("{}", s);

    // Mutable reference
    let mut s = String::from("Merhaba ");
    let mut_ref = &mut s;

    mut_ref.push_str("Dunya");
    println!("{}", s);
}
// Ownership and multiple variables

// Deep copy -> clone()

fn main()
{
    let s1 = String::from("Merhaba"); // s1 is the owner of the string "Merhaba"
    //let s2 = s1; // s2 is the owner of the string "Merhaba"
    let s2 = s1.clone(); // new copy of the String "Merhaba" is created and s2 is the owner of the new copy

    println!("{} {}", s2, s1); // s1 and s2 are the owners of the string "Merhaba"
}
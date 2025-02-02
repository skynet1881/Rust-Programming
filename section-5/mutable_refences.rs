// Mutable reference in Rust
// In Rust, you can also create mutable references using &mut keyword for non-primitive data types. This allows you to modify the value being referred to.

fn main()
{
    let mut s = String::from("Merhaba"); // complex type, s is mutable
    let s_ref = &mut s; // reference to s, borrowing, s_ref is mutable
    s_ref.push_str(" Dunya"); // error: cannot borrow `*s_ref` as mutable, as it is behind a `&` reference

    println!("s_ref = {}", s_ref ); // both can be used
}
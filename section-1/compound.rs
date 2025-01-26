// Compound data types in Rust
// Arrays, Tuples, Slices and strings

// Arrays
fn main()
{
    let arr = [1, 2, 3, 4, 5];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array: {:?}", arr);

    let fruits = ["apple", "banana", "mango"];
    println!("Fruits: {}", fruits[0]);
    println!("Fruits: {}", fruits[1]);
    println!("Fruits: {}", fruits[2]);

    // Tuples
    let insan1: (String, i32, bool) = ("Melis".to_string(), 25, true);
    let insan2: (&str, i32, bool) = ("Can", 32, false);
    println!("Name: {}", insan1.0);
    println!("Age: {}", insan1.1);
    println!("Dogruluk: {}", insan1.2);

    println!("Name: {}", insan2.0);

    // Slices: number slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);

    // Strings
    let name = "Melis";
    let mut ulke: String = "Turkey".to_string();
    println!("Name: {}", name);
    println!("Country: {}", ulke);
    ulke.push_str(" is a beautiful country");
    println!("Country: {}", ulke);
}
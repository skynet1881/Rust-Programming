// Compound data types in Rust
// Arrays, Tuples, Slices, strings

fn main()
{
    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let numbers: [u32; 5] = [1, 2, 3, 4, 5];
    let empty: [u32; 0] = [];

    println!("Array: {:?}", numbers);

    let fruits = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);

    // Tuples
    let insan1: (String, i32, bool) = ("Melis".to_string(), 25, true);
    let insan2: (&str, i32, bool) = ("Can", 32, false);
    println!("Name: {}", insan1.0);
    println!("Age: {}", insan1.1);
    println!("Is it correct: {}", insan1.2);

    println!("Name: {}", insan2.0);
    println!("Age: {}", insan2.1);
    println!("Is it correct: {}", insan2.2);

    // Slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    
    println!("Slice: {:?}", slice);

    // Strings
    let name = "Melis";
    let mut country: String = "Turkey".to_string();
    
    println!("Name: {}", name);
    println!("Country: {}", country);

    country.push_str(" is a beautiful country");
    println!("Country: {}", country);
}
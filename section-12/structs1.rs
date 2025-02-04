// Section 12 - Structs in Rust
// Why we need structs in Rust
// They allow us to group related data together


struct Insan {
    name: String,
    age: u32,
    height: f32,
    weight: f32,
}

fn main()
{
    // Creating a new instance of the struct
    let person1 = Insan {
        name: String::from("Can"),
        age: 32,
        height: 1.75,
        weight: 80.0,
    };

    let person2 = Insan {
        name: String::from("Berker"),
        age: 18,
        height: 1.60,
        weight: 60.0,
    };

    println!("{} is {} years old, {} meters tall and weighs {} kg.", person1.name, person1.age, person1.height, person1.weight);
    println!("{} is {} years old, {} meters tall and weighs {} kg.", person2.name, person2.age, person2.height, person2.weight);
}
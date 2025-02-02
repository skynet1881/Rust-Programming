// Difference of constants with variables 

fn main()
{
    // 1. Mutable variables
    let mut yas: u32 = 30;
    println!("yas: {}", yas);

    yas = 31; // yas is mutable, so we can change value
    println!("yas: {}", yas);

    // 2. Immutable variables
    let name: &str = "Merhaba";
    println!("name: {}", name);

    // name = "Dunya";

    // 3. Constants; can not be modified
    const FAIZ_ORANI: f64 = 0.75;
    println!("FAIZ_ORANI: {}", FAIZ_ORANI);

    // FAIZ_ORANI = 0.5; // Can not assign value twice

    // DIFFERENCE OF CONSTANTS WITH VARIABLES
    // Constants use const, variables use let
    // Constants must have type
    // Constants are immutable
    // Constants are created at compile time, variables are assigned at runtime
}
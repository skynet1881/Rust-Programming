// Functions in Rust

fn main() {
    println!("Hello, world!");

    // Function call
    another_function();

    // Sum function
    sum(10, 20);

    // bmi function
    let weight = 81.0;
    let height = 1.75;
    let bmi = bmi(weight, height);
    println!("BMI: {}", bmi);
}

fn another_function() {
    println!("Another function.");
}

fn sum(a: i32, b: i32) {
    println!("Sum of {} and {} is {}", a, b, a + b);
}

fn bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}
// Functions in Rust

fn main()
{
    // basics of functions
    print_hello();
    another_function();

    // sum of numbers
    sum(10, 20);
    sum(-10, 10);
    sum(0, 0);  

    // subtraction of numbers
    sub(10, 20);
    sub(-10, 10);
    sub(20, 10);

    // bmi calculation
    let weight = 81.0;
    let height = 1.75;
    let bmi = bmi(weight, height);
    println!("BMI: {}", bmi);
}

fn print_hello()
{
    println!("Hello, World!");
}

fn another_function()
{
    println!("Another function");
}

fn sum(a: i32, b: i32)
{
    println!("Sum of {} and {} is {}", a, b, a + b);
}

fn sub(a: i32, b: i32)
{
    println!("Subtraction of {} and {} is {}", a, b, a - b);
}

fn bmi(weight: f32, height: f32) -> f32
{
    weight / (height * height)
}
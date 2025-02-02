// Functions in Rust

fn main()
{
    // basic function call
    print_hello();
    another_function();

    // sum function
    sum(10, 20);
    sum(100, 200);
    sum(-10, -20);

    // subtranction function
    subraction(10, 20);
    subraction(100, 200);
    subraction(-10, -20);

    // bmi functin
    let weight1 = 81.0;
    let height1 = 1.75;
    let bmi1 = bmi(weight1, height1);
    println!("BMI of weight {} and height {} is {}", weight1, height1, bmi1);

    let weight2 = 65.0;
    let height2 = 1.75;
    let bmi2 = bmi(weight2, height2);
    println!("BMI of weight {} and height {} is {}", weight2, height2, bmi2);
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
    println!("Sum of {} and {} is {}", a, b, a+b);
}

fn subraction(a: i32, b: i32)
{
    println!("Subraction of {} and {} is {}", a, b, a-b);
}

fn bmi(weight: f32, height: f32) -> f32
{
    weight / (height * height)
}
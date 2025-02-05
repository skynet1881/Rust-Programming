// Lesson 2: Functions parameters

// Functions can take parameters, which are values that are passed into the function.
// Each parameter must be explicitly typed, and the type of the parameter must be specified in the function signature.

fn add(a: i32, b: i32) -> i32
{
    a + b
}

fn main()
{
    let result = add(-5, -10);
    println!("The result is: {}", result);
}
// Lesson 4: Function with Multiple Return Values using Tuples

// Functions can return multiple values using tuples.

fn swap(x: i32, y: i32) -> (i32, i32)
{
    (y, x)
}

fn main()
{
    let (a, b) = swap(-50 , 10);
    println!("a: {}, b: {}", a, b);
}
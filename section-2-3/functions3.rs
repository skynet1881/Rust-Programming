// Lesson 3: Function Return Values

// The return type of a function must be specified using `->`.
// The last expression in the function body is the return value (without `return` keyword).

fn square(number: i32) -> i32
{
    number * number
}

fn main()
{
    let sq = square(5);
    println!("The square of 0 is {}", sq);
}
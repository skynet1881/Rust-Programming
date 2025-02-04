// Loops in Rust
// Why loops are important?
// Loops are used to execute a block of code multiple times. It is used to iterate over a sequence of elements.

// 1. loop

fn main()
{
    let mut counter = 0;

    loop
    {
        println!("Counter: {}", counter);
        counter += 1; // mutable variable

        if counter == 10
        {
            println!("Counter reached 10 - break loop");
            break;
        }
    }

    println!("Loop ends here");  
}
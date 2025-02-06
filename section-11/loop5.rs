// break and continue in loop

/*
    loop
    {
        if condition
        {
            break;
        }
    }
*/

fn main()
{
    for i in 1..=10
    {
        if i == 3
        {
            println!("I am {}", i);
            continue; // jumps to next iteration
        }
        if i == 9
        {
            println!("I am {}", i);
            break; // break the loop
        }
        println!("Procesing {}", i);
    }

    println!("End of loop");
}
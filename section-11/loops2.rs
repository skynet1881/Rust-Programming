// loop in Rust

fn main()
{
    let mut counter = 0;

    loop
    {
        println!("Counter {}", counter);
        counter += 1;

        if counter >= 10
        {
            println!("Counter reached {}", counter);
            break;
        }
    }
}

// loop
// while
// for

fn loop_1()
{
    println!("Loop 1");
    let mut counter = 0;
    loop
    {
        counter += 1;
        println!("Counter {}", counter);

        if counter == 10
        {
            break;
        }
    }
}

fn loop_2()
{
    println!("Loop 2");
    let mut counter = 0;
    while counter < 10
    {
        counter += 1;
        println!("Counter {}", counter);
    }
}

fn loop_3()
{
    println!("Loop 3");
    for counter in 1..=10
    {
        println!("Counter {}", counter);
    }
}   

fn main()
{
    loop_1();
    loop_2();
    loop_3();
}

/*fn main()
{
    let mut number = 5;

    loop
    {
        println!("Number is: {}", number);
        number -= 1;

        if number == 0
        {
            println!("Loop ends here");
            break;
        }
    }
}*/

fn main()
{
    let mut number = 5;

    while number != 0
    {
        println!("Number is: {}", number);
        number -= 1;
    }
    println!("Loop ends here");
}
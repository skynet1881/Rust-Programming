// Logic operations in condtions

// && and
// || or
// ! not

fn main()
{
    let x = 5;
    let y = -1;

    // && and check
    if x > 0 && y > 0
    {
        println!("x ve y pozitif");
    }

    // || or check
    if x < 0 || y < 0
    {
        println!("x veya y negatif");
    }

    let condition = true;
    // ! not check
    if !condition 
    {
        println!("condition false");
    }
}
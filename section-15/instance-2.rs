struct Car 
{
    brand: String,
    year: u16,
    color: String,
}

fn main()
{
    let car1 = Car
    {
        brand: String::from("Toyota"),
        year: 2010,
        color: String::from("Black"),
    };

    let car2 = Car
    {
        brand: String::from("BMW"),
        year: 2022,
        color: String::from("Gray"),
    };

    let car3 = Car
    {
        brand: String::from("Audi"),
        year: 2020,
        color: String::from("White"),
    };

    println!("Car 1: {} {} {}", car1.brand, car1.year, car1.color);
    println!("Car 2: {} {} {}", car2.brand, car2.year, car2.color);
    println!("Car 3: {} {} {}", car3.brand, car3.year, car3.color);
}

// Conditions devam 3

fn main()
{
    // sayi tek mi cift mi
    let number = 42;

    let result = if number % 2 == 0 { "Cift sayi"} else { "Tek sayi"};
    println!("{} sayisi {}", number, result); 
}
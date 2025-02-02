// Rust primitive data types - temel degiskenler
// 1. Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
// unsigned + ve signed integer + - değerler alabilir

// 2. Float: f32, f64

// 3. Boolean: bool true veya false değer alabilir

// 4. Char: char tek tırnak içinde bir karakter alabilir
fn main()
{
    let sayi: i32 = -10;
    println!("Sayi: {}", sayi);

    let sayi2: u32 = 20;
    println!("Sayi2: {}", sayi2);

    let fsayi1: f32 = 10.5;
    println!("Float Sayi1: {}", fsayi1);

    let fsayi2: f64 = 20.5;
    println!("Float Sayi2: {}", fsayi2);

    let is_true: bool = true;
    println!("Is True: {}", is_true);

    let is_false: bool = false;
    println!("Is False: {}", is_false);

    let karakter: char = '1';
    println!("Karakter: {}", karakter);

    let karakter2: char = 'A';
    println!("Karakter2: {}", karakter2);
}
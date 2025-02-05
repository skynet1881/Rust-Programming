// Lesson 1: Basics of Enums
// In Rust, an `enum` (short for enumeration) is a way to define a type
// that can have multiple different possible values.

enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

fn main()
{
    let movement = Direction::Left; // This is how you create an instance of an enum.

    match movement{
        Direction::Up => println!("Movement up"),
        Direction::Down => println!("Movement down"),
        Direction::Left => println!("Movement left"),
        Direction::Right => println!("Movement right"),
    }
}
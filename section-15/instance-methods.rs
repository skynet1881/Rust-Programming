// impl use for instance methods

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn is_big(&self) -> bool
    {
        self.area() > 64
    }
}

fn main()
{
    // create instances of Rectangle
    let rec1 = Rectangle{width: 5, height: 10};
    let rec2 = Rectangle{width: 10, height: 8};

    println!("Area of rec1: {} and is_big: {}", rec1.area(), rec1.is_big());
    println!("Area of rec2: {} and is_big: {}", rec2.area(), rec2.is_big());
}
// Ownership in vectors

fn main()
{
    let v = vec![1, 2, 3, 4, 5]; // v owns the vector

    let v2 = v; // v2 takes ownership of the vector
    // v is no longer valid here
    println!("{:?}", v2);
}
// Lesson 1: Introduction to Comments in Rust

// Rust supports two main types of comments: 
// 1. Line comments (single-line) using `//`
// 2. Block comments (multi-line) using `/* ... */`

// Line comments: Everything after `//` on a line is ignored by the compiler
// This is a simple line comment explaining the following line of code:
let x = 5; // Assigning the value 5 to variable x

// Block comments: These can span multiple lines
/*
 This is a block comment.
 It can be used to write longer explanations.
 Everything within `/* ... */` is ignored by the compiler.
*/

// Block comments are useful when explaining complex logic.
/*
let y = x + 10; // This would add 10 to x
let z = y * 2;  // This would multiply y by 2
*/

// In general, use `//` for short explanations and `/* */` for longer notes.

// magic number is used for stride calculation
let magic_number = 15;

// this buffer size used for GPU operations
let buffer_size = 1024;

/*
    Sum of two integer
    a: first integer
    b: second integer
    return: sum of a and b
*/
fn sum(a: i32, b: i32) -> i32 {
    a + b
}


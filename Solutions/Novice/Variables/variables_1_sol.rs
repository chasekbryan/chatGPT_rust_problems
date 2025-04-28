// variables_1.rs
// Type: Variables
// Difficulty: Novice
//
// Lesson:
// In Rust, variables are immutable by default. To allow modification, you must opt into mutability with `mut`.
// Use `println!` to output values. This exercise will practice declaring immutable and mutable variables,
// reassigning a mutable variable, and printing both values.
//
// Instructions:
// 1. Declare an immutable variable `x` initialized to 10.
// 2. Declare a mutable variable `y` initialized to 5.
// 3. Reassign `y` to 15.
// 4. Print `x` and `y` in the format: `x = {x}, y = {y}`.
//
// Hints:
// - `let x = ...;`
// - `let mut y = ...;`
// - `println!("x = {}, y = {}", x, y);`
fn main() {
    // DONE: Declare an immutable variable x with value 10.
    let x = 10;
    // TODO: Declare a mutable variable y with value 5.
    let y = 5;
    // TODO: Reassign y to value 15.
    let y = 15;
    // TODO: Print x and y.
    println!("x: {x}\ny: {y}");
}


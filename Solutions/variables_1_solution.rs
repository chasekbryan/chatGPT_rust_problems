// variables_1.rs
// Type: Variables
// Difficulty: Novice (Level 1)
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
    // DONE: Declare a mutable variable y with value 5.
    let mut y = 5;  // this variable has a warning because it was never used
    // DONE: Reassign y to value 15.
    y = 15;
    // DONE: Print x and y.
    println!("x = {}, y = {}", x, y);
}


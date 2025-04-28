// functions_16.rs
// Type: Functions
// Difficulty: Novice (Level 1)
//
// Lesson:
// Functions can borrow values instead of taking ownership by using references (`&`). 
// This allows the caller to continue using the original value after the function call.
//
// Instructions:
// 1. Define a function named `print_borrowed` that takes a reference to a `String` (`&String`).
// 2. It should print the string without taking ownership.
// 3. In `main`, create a `String`, pass a reference to `print_borrowed`, and then print the `String` again.
//
// Hints:
// - Use `&` in both the parameter type and when passing the argument to the function.
// - Borrowed references allow reuse of the original value.

fn main() {
    // TODO: Create a String, pass a reference to print_borrowed, then print the String again
}

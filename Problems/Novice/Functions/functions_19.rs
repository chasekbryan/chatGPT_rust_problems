// functions_19.rs
// Type: Functions
// Difficulty: Novice (Level 1)
//
// Lesson:
// Functions can return `Result` types to indicate success or failure, which is very common in Rust for error handling.
//
// Instructions:
// 1. Define a function named `safe_divide` that takes two `i32` parameters: numerator and denominator.
// 2. If the denominator is zero, return an `Err` with a string error message.
// 3. Otherwise, return `Ok(result)` where `result` is the numerator divided by denominator.
// 4. In `main`, call `safe_divide` with 8 and 0 and print the result.
//
// Hints:
// - `Result` is an enum: `Ok(value)` or `Err(error_message)`.
// - You can return a `Result<i32, String>`.
// - Use `match` in `main` to handle `Ok` and `Err`.

fn main() {
    // TODO: Call safe_divide with 8 and 0, then use match to print the outcome
}

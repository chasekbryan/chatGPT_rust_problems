// arithmetic_1_sol.rs
// Type: Arithmetic
// Difficulty: Novice
//
// Lesson:
// In Rust, you can perform basic arithmetic (+, -, *, /) directly on numeric types.
// Integer division truncates toward zero. To get a floating-point result, use a float type.
// This exercise will practice declaring numeric variables, doing arithmetic operations,
// and printing each result.
//
// Instructions:
// 1. Declare two integer variables, `a` = 12 and `b` = 5.
// 2. Compute their sum, difference, product, and integer quotient, storing each in its own variable.
// 3. Declare two floating-point variables, `x` = 12.0 and `y` = 5.0.
// 4. Compute the floating-point quotient `xfq = x / y`.
// 5. Print all results in the format:
//    “a + b = {sum}”, “a - b = {diff}”, “a * b = {prod}”, “a / b = {quot}”
//    “x / y = {xfq}”
//
// Hints:
// - Integer division: `let quot = a / b;`
// - Float division: `let xfq = x / y;`
// - Print with `println!("... {}", var);`
fn main() {
    // DONE: 1. Declare integer variables a and b.
    // let a = 12;
    // let b = 5;
    let a = 12;
    let b = 5;
    // DONE: 2. Compute sum, difference, product, quotient.
    // let sum = a + b;
    // let diff = a - b;
    // let prod = a * b;
    // let quot = a / b;
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    // DONE: 3. Declare float variables x and y.
    // let x = 12.0;
    // let y = 5.0;
    let x = 12.0;
    let y = 5.0;
    // DONE: 4. Compute floating-point quotient.
    // let xfq = x / y;
    let xfq = x / y;
    // DONE: 5. Print all results.
    // println!("a + b = {}", sum);
    // println!("a - b = {}", diff);
    // println!("a * b = {}", prod);
    // println!("a / b = {}", quot);
    // println!("x / y = {}", xfq);
    println!("a + b = {}", sum);
    println!("a - b = {}", diff);
    println!("a * b = {}", prod);
    println!("a / b = {}", quot);
    println!("x / y = {}", xfq); 
}

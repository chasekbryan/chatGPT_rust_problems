// variables_2.rs
// Type: Variables
// Difficulty: Novice
//
// Lesson:
// In Rust, you can create new variables by using `let` and you can 
// shadow existing variables by re-declaring them with `let`. 
// Shadowing allows you to re-use the same variable name but change 
// its type or value safely.
// 
// In this exercise, practice basic shadowing and updating variables.

fn main() {
    let x = 5;
    
    // TODO: Shadow `x` here by adding 1 to its value.
    let x = x + 1;

    // TODO: Shadow `x` again by multiplying it by 2.

    println!("The final value of x is: {x}");
}

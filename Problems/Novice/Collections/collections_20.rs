// collections_20.rs
// Type: Collections
// Difficulty: Novice (Level 1)
//
// Lesson:
// You can create a `Vec` that holds elements of different types using `enum` or a trait object like `Box<dyn Any>`.
// However, `Vec` only supports one type at a time by default.
//
// Instructions:
// 1. Create a vector of `Box<dyn std::any::Any>` and store different types of data like `i32` and `String`.
// 2. Print the contents of the vector by checking the type at runtime.
//
// Hints:
// - Use `Box::new` to create a boxed value.
// - You can check the type using `.downcast_ref::<Type>()` on the `Box` object.

fn main() {
    // TODO: Create a vector and print its contents by checking the type at runtime
}

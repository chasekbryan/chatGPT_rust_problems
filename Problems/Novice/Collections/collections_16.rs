// collections_16.rs
// Type: Collections
// Difficulty: Novice (Level 1)
//
// Lesson:
// You can filter elements from a vector using `.iter().filter()` and collect the results into a new `Vec`.
//
// Instructions:
// 1. Create a `Vec` named `numbers` with `[1, 2, 3, 4, 5, 6]`.
// 2. Create a new vector that only contains the even numbers.
// 3. Print the new vector.
//
// Hints:
// - Use `.iter().filter(|&&x| x % 2 == 0).collect::<Vec<_>>()`.
// - Remember to dereference (`&&x`) inside the filter when iterating over references.

fn main() {
    // TODO: Filter even numbers and print the new vector
}

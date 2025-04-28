// conditionals_20.rs
// Type: Conditionals
// Difficulty: Novice (Level 1)
//
// Lesson:
// Pattern matching can destructure nested types, such as enums containing other enums or structs.
//
// Instructions:
// 1. Define two enums: `Status` (`Online`, `Offline`) and `User` with fields `name` (String) and `status` (Status).
// 2. Create a `User` instance named `user1`.
// 3. Use a match statement:
//    - If `user1` is `Online`, print "{name} is online.".
//    - If `user1` is `Offline`, print "{name} is offline.".
//
// Hints:
// - Match inside the struct, for example: `User { name, status: Status::Online }`.
// - Be careful to destructure fields properly and handle all cases.

fn main() {
  // TODO: Define enums `Status` and `User`.

  // TODO: Create a `user1` instance.

  // TODO: Write a match statement to destructure nested types.
}
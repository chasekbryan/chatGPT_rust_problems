# Apprentice Rust Learning Guide

This guide outlines the plan for the **Apprentice** level of Rust coding exercises. It builds directly on the skills you developed in the **Novice** portion, introducing more complex patterns, deeper understanding of Rust fundamentals, and preparing you for truly intermediate programming challenges.

Each exercise will still follow the established format: a clearly written problem prompt, a mini-lesson, detailed instructions, and hints. As before, no solutions will be included inside the exercise files; solutions will be stored separately.

---

## Apprentice Structure

- **Exercises per Topic**: 75 exercises for each topic.
- **Directory Layout**:
  ```
  /problems
    apprentice/
      variables/
      arithmetic/
      conditionals/
      loops/
      functions/
      tuples_and_arrays/
      strings_and_slices/
      collections/
      ownership_and_borrowing/
      error_handling/
      structs_and_enums/
      modules/
  ```
- **Problem File Naming**: `topic_X.rs`, where `topic` is the folder name and `X` is the exercise number (e.g., `variables_21.rs`, `ownership_and_borrowing_1.rs`).

## Topics Covered in Apprentice

| Topic                   | Description                                                     |
| ----------------------- | --------------------------------------------------------------- |
| Variables               | Mutability patterns, shadowing, scopes, lifetimes introduction  |
| Arithmetic              | More complex expressions, overflow handling, custom types       |
| Conditionals            | Nested conditionals, logical operators chaining                 |
| Loops                   | Complex nested loops, loop labels, breaking with values         |
| Functions               | Passing by reference, returning tuples, early returns           |
| Tuples and Arrays       | Complex destructuring, multi-dimensional arrays                 |
| Strings and Slices      | Advanced slicing, formatting macros, string ownership           |
| Collections             | Vecs, HashMaps, iteration patterns, collection manipulation     |
| Ownership and Borrowing | Core Rust concept: move semantics, references, lifetimes        |
| Error Handling          | Result type, panic! handling, error propagation with ? operator |
| Structs and Enums       | Defining custom types, method implementations, pattern matching |
| Modules                 | Organizing code, visibility (pub), using mod.rs structure       |

---

## Suggested Learning Strategy

1. **Warm-Up:**

   - Review your Novice solutions and spot where you struggled.
   - Start by doing a few early Apprentice problems from "Variables" to reacclimate to problem-solving.

2. **Follow the Sequential Order:**

   - Tackle topics in this order:
     1. Variables
     2. Arithmetic
     3. Conditionals
     4. Loops
     5. Functions
     6. Tuples and Arrays
     7. Strings and Slices
     8. Collections
     9. Ownership and Borrowing
     10. Error Handling
     11. Structs and Enums
     12. Modules

3. **Push Through Struggle:**

   - Expect exercises to be harder.
   - Take time to understand compiler errors; they are extremely important now.

4. **Use Hints Strategically:**

   - Hints are given to guide thought, not to substitute effort.
   - Try to "debug" hints the way you would debug code.

5. **Code Consistently:**

   - Set a goal: for example, 2-3 exercises per day.
   - Practice frequency matters much more than session length.

6. **Reflect Often:**

   - After every 10-15 problems, review:
     - What did you find easy?
     - What patterns are starting to feel natural?
     - What still confuses you?

7. **Test Your Code:**

   - Begin writing `assert_eq!` test cases in your problems as a habit.
   - Thinking in tests will prepare you for professional development work.

---

## Key Apprentice Goals

- Develop strong **ownership and borrowing** instincts.
- Be comfortable handling **errors** safely.
- Organize larger codebases using **modules**.
- Design and implement custom data types using **structs and enums**.
- Prepare for writing "real-world" Rust projects after Apprentice level.

---

## Final Note

The Apprentice portion will be more demanding, but it is designed to sharpen you for real Rust development. Mastering these concepts will make tackling external projects, open-source contributions, and technical interviews far easier.

Stay consistent, challenge yourself to solve problems without rushing to hints or solutions, and maintain curiosity.

Let's keep moving forward!

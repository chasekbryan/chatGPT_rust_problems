# chatGPT_rust_problems

## Overview
This repository contains a guided and progressive set of Rust coding exercises. It is designed to help learners build a strong foundation in Rust, starting from basic topics and advancing gradually to more complex concepts. Each exercise is structured to encourage practice without relying on immediate solutions.

## Directory Structure
```
/Admin
  prompt_understanding.txt  # Document explaining project structure and guidelines

/problems
  novice/
    variables/              # Up to 20 problem files: variables_X.rs
    arithmetic/             # Up to 20 problem files: arithmetic_X.rs
    conditionals/           # Up to 20 problem files: conditionals_X.rs
    loops/                  # Up to 20 problem files: loops_X.rs
    functions/              # Up to 20 problem files: functions_X.rs
    tuples_and_arrays/      # Up to 20 problem files: tuples_and_arrays_X.rs
    strings_and_slices/     # Up to 20 problem files: strings_and_slices_X.rs
    collections/            # Up to 20 problem files: collections_X.rs

  apprentice/               # More advanced problems, same structure
  neophyte/                 # Higher difficulty problems
  adept/                    # Most advanced problems in this stage
```

## Problem File Format
- File naming: `topic_X.rs` (e.g., `variables_1.rs`, `arithmetic_2.rs`)
- Each file includes:
  - **Type:** Topic
  - **Difficulty:** Level (e.g., Novice)
  - **Lesson:** Concept overview
  - **Instructions:** Clear, numbered steps for the learner
  - **Hints:** Helpful guidance without giving away the answer
- Files contain a `fn main()` with `// TODO` markers for where learners write their solutions.
- Solutions are stored separately, not inside the problem files.

## Prompt and Generation Rules
- Work sequentially: complete one exercise before moving to the next.
- Each new exercise builds slightly upon previous knowledge.
- Maintain consistent file naming and directory structure.
- Include helpful hints, but never full solutions inside problem files.
- Progress from novice topics to apprentice, neophyte, and adept tiers.

## Novice Topic Categories
1. Variables
2. Arithmetic (integer and floating-point operations)
3. Conditionals (`if`/`else`)
4. Loops (`for` and `while` loops)
5. Functions (definition, parameters, and return values)
6. Tuples and Arrays (access and destructuring)
7. Strings and Slices (basic manipulation and formatting)
8. Collections (creating and iterating over `Vec`)

## Future Topics (for higher levels)
- Ownership and Borrowing
- Error Handling
- Structs and Enums
- Modules and Organization
- Advanced Rust Patterns

Problem files will always prioritize hands-on practice by only including the prompt, lesson, instructions, and hints.

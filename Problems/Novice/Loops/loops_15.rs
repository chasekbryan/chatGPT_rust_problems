// loops_15.rs
// Type: Loops
// Difficulty: Novice (Level 1)
//
// Lesson:
// In Rust, you can label loops and then `break` out of an outer loop from inside an inner loop
// by using `break 'label_name;`. This is useful when dealing with nested loops.
//
// Instructions:
// 1. Create an outer `loop` labeled `'outer`.
// 2. Inside it, create an inner `loop`.
// 3. Break out of the inner loop when an inner counter reaches 2.
// 4. Break out of the outer loop when an outer counter reaches 3.
// 5. Print messages showing the current loop levels and counters.
//
// Hints:
// - Label a loop like `'outer: loop {}`.
// - Use separate mutable counters for inner and outer loops.
// - Remember to reset the inner counter each time the outer loop iterates.

fn main() {
    // TODO: Create a mutable outer counter starting at 0
    
    'outer: loop {
        // TODO: Create a mutable inner counter starting at 0
        
        loop {
            // TODO: Print outer and inner counter values
            
            // TODO: Break inner loop when inner counter reaches 2
            
            // TODO: Increment inner counter
        }
        
        // TODO: Increment outer counter
        
        // TODO: Break outer loop when outer counter reaches 3
    }
}

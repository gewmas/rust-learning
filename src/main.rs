fn main() {
    // Phase 1: Ownership and Moves
    // In Rust, memory is managed by a system of ownership.

    let s1 = String::from("hello"); // s1 owns the String

    let s2 = s1; // The ownership is MOVED to s2. s1 is no longer valid.

    // UNCOMMENT the line below and run `cargo check` to see the error.
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    // EXERCISE: Why does this happen?
    // In JS/Python, s1 and s2 would both point to the same object.
    // In Rust, only one variable can "own" a piece of heap memory at a time.
    // This prevents "double free" errors.
}

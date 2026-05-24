pub fn run() {
    println!("\n--- Borrowing & References ---");

    let mut s1 = String::from("hello");

    // 1. Immutable borrowing
    let len = calculate_length(&s1); // We pass a reference
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here!

    // 2. Mutable borrowing
    change(&mut s1);
    println!("Modified string: {}", s1);

    // 3. THE GOLDEN RULE EXERCISE
    // You cannot have a mutable reference while you have immutable ones.
    // UNCOMMENT the block below to see the borrow checker in action:

    let r1 = &s1; // immutable borrow
    let r2 = &s1; // immutable borrow
                  // let r3 = &mut s1; // PROBLEM: mutable borrow
                  // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it's a reference, nothing happens to the original.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn run() {
    println!("\n--- Error Handling ---");

    // 1. Handling Option (Something or Nothing)
    let fruits = vec!["apple", "banana", "coconut"];
    let first = fruits.get(0);
    println!("First fruit: {:?}", first);

    // 2. Handling Result (Success or Error)
    let outcome = divide(10.0, 2.0);
    match outcome {
        Ok(value) => println!("Division result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // 3. The '?' Operator (The idiomatic way)
    match complex_operation() {
        Ok(_) => println!("Complex operation succeeded!"),
        Err(e) => println!("Complex operation failed: {}", e),
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn complex_operation() -> Result<(), String> {
    // The '?' operator returns the error early if it's an Err
    let _val1 = divide(10.0, 2.0)?;
    let _val2 = divide(20.0, 0.0)?; // This will trigger an early return

    Ok(()) // Only reached if both above succeeded
}

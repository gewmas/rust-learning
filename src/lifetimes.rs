pub fn run() {
    println!("\n--- Lifetimes: Signature vs Body ---");

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        // This still works because result is used ONLY inside this scope
        // result = longest(string1.as_str(), string2.as_str());
        result = always_returns_first(string1.as_str(), string2.as_str());
        println!("Inside scope: {}", result);
    }

    // THIS WORKS!
    // Even though string2 is dead, the function signature promised
    // that the result ONLY depends on string1's lifetime.
    println!("Outside scope: {}", result);

    // KEY TAKEAWAY:
    // The compiler looks at the SIGNATURE, not the BODY, to decide safety.
}

// Even though we take 'y', we don't link its lifetime to the return value.
// The compiler only cares about 'x' for the output's validity.
fn always_returns_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn always_returns_second<'a>(x: &str, y: &'a str) -> &'a str {
    y
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

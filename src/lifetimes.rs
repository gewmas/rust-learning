pub fn run() {
    println!("\n--- Lifetimes: Signature vs Body ---");

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = always_returns_first(string1.as_str(), string2.as_str());
        println!("Inside scope: {}", result);
    }

    println!("Outside scope: {}", result);
}

fn always_returns_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// These are kept for reference but prefixed with underscores to silence warnings
fn _always_returns_second<'a>(_x: &str, y: &'a str) -> &'a str {
    y
}

fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

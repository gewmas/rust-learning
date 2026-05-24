pub fn run() {
    println!("\n--- Lifetimes ---");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // EXERCISE: Understanding the 'Why'
    // Lifetimes are just a way for the compiler to ensure that references
    // don't outlive the data they point to (preventing "dangling pointers").
}

// The generic lifetime 'a says:
// "The returned reference will live at least as long as the SHORTER of x and y."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

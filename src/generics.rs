use crate::traits::{NewsArticle, Summary, Tweet};

pub fn run() {
    println!("\n--- Generics ---");

    let tweet = Tweet {
        username: String::from("rust_dev"),
        content: String::from("Generics + Traits = Power"),
    };

    let article = NewsArticle {
        headline: String::from("Generic Programming"),
        author: String::from("Bjarne Stroustrup (Wait, wrong language)"),
    };

    // 1. Using a Generic Function
    notify(&tweet);
    notify(&article);

    // 2. Using a Generic Struct
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };

    println!("Point 1: ({}, {})", p1.x, p1.y);
    println!("Point 2: ({}, {})", p2.x, p2.y);
}

// A Generic Function with a "Trait Bound"
// This says: "I can take any type T, as long as T implements Summary."
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// A Generic Struct
struct Point<T> {
    x: T,
    y: T,
}

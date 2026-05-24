pub fn run() {
    println!("\n--- Traits ---");

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Traits are powerful!"),
    };

    let article = NewsArticle {
        headline: String::from("Rust 1.0 Released"),
        author: String::from("The Rust Team"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    // Using a default implementation
    println!("Default summary: {}", tweet.summarize_default());
}

// 1. Define a Trait (like an Interface)
pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

// 2. Implement the Trait for a specific type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

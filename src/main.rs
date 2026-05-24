mod borrowing;
mod lifetimes;
mod ownership;

fn main() {
    println!("Welcome to Rust Mastery!");

    // Run the completed exercises
    ownership::run();
    borrowing::run();
    lifetimes::run();
}

mod borrowing;
mod enums;
mod generics;
mod lifetimes;
mod ownership;
mod traits;

fn main() {
    println!("Welcome to Rust Mastery!");

    // Run the completed exercises
    ownership::run();
    borrowing::run();
    lifetimes::run();
    enums::run();
    traits::run();
    generics::run();
}

pub fn run() {
    println!("--- Ownership & Moving ---");

    let s1 = String::from("hello");
    let s2 = s1; // Moved ownership

    // println!("{}, world!", s1); // Error!
    println!("s2: {}", s2);

    // Cloning (if we want both)
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}

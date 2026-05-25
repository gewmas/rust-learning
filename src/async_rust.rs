use tokio::time::{sleep, Duration};

pub async fn run() {
    println!("\n--- Async Rust ---");

    // 1. Basic async/await
    let future = say_hello(); // This doesn't run yet!
    println!("Future created, now awaiting...");
    future.await; // NOW it runs

    // 2. Concurrent execution with join!
    println!("Starting concurrent tasks...");
    let start = std::time::Instant::now();

    let task1 = task_one();
    let task2 = task_two();

    // Both tasks run concurrently
    tokio::join!(task1, task2);

    let duration = start.elapsed();
    println!(
        "Tasks finished in {:?}. (Notice it's not 3 seconds!)",
        duration
    );
}

async fn say_hello() {
    println!("Hello from the async world!");
}

async fn task_one() {
    sleep(Duration::from_secs(1)).await;
    println!("Task One finished (took 1s)");
}

async fn task_two() {
    sleep(Duration::from_secs(2)).await;
    println!("Task Two finished (took 2s)");
}

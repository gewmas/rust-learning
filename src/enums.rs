pub fn run() {
    println!("\n--- Enums & Pattern Matching ---");

    let my_web_event = WebEvent::KeyPress('q');
    let my_status = Status::Error(404);

    inspect_event(WebEvent::PageLoad);
    inspect_event(WebEvent::PageUnload);
    inspect_event(my_web_event);
    inspect_event(WebEvent::Paste("Hello, Rust!".to_string()));

    inspect_status(Status::Success);
    inspect_status(my_status);
}

// Rust enums are "Sum Types" (or ADTs).
// They can hold data, unlike C-style enums.
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
}

enum Status {
    Success,
    Error(u32),
}

fn inspect_event(event: WebEvent) {
    // 'match' is exhaustive. The compiler forces you to handle ALL cases.
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: \"{}\"", s),
    }
}

fn inspect_status(status: Status) {
    if let Status::Error(code) = status {
        println!("Detected an error with code: {}", code);
    } else {
        println!("All systems go!");
    }
}

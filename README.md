# Rust Mastery for Full-Stack Veterans

Welcome to your Rust journey. Since you have 10 years of experience, we won't spend time on "what is a variable." Instead, we will focus on the unique memory model and type system that makes Rust powerful.

## 🗺 Roadmap

### Phase 1: The Memory Model (The Hurdle)
- [ ] **Ownership & Moving:** Understanding how Rust manages memory without a GC.
- [ ] **Borrowing & References:** `&T` vs `&mut T`. The golden rule: one mutable XOR many immutable.
- [ ] **Lifetimes:** Telling the compiler how long data lives.

### Phase 2: Power Typing
- [ ] **Enums & Pattern Matching:** Using ADTs to represent state (e.g., `Option`, `Result`).
- [ ] **Traits:** Defining shared behavior (Rust's take on interfaces).
- [ ] **Generics:** Writing reusable code without performance hits (monomorphization).

### Phase 3: Modern Backend Rust
- [ ] **Error Handling:** Propagating errors with the `?` operator.
- [ ] **Async Rust:** The `Tokio` runtime and `Future` trait.
- [ ] **Serialization:** Using `Serde` for JSON/Data handling.

### Phase 4: Full-Stack Integration
- [ ] **Web Frameworks:** Building an API with `Axum` or `Actix-Web`.
- [ ] **Persistence:** Connecting to Postgres with `SQLx` or `Diesel`.
- [ ] **WASM:** Compiling Rust for the browser.

## 🚀 Getting Started
Your first exercise is in `src/main.rs`. We'll start by breaking the borrow checker to understand it.

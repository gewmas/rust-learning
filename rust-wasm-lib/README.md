# Rust WASM Library

This library provides high-performance Rust functions to be used in a frontend application (like React).

## 🛠 Prerequisites

Ensure you have the following installed:
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A compatible version of `wasm-bindgen-cli` (installed via `cargo install wasm-bindgen-cli --version 0.2.100 --locked`)

## 🚀 Build Instructions

To compile the Rust code into a WebAssembly package:

```bash
# Navigate to this directory
cd rust-wasm-lib

# Build for the web
wasm-pack build --target web
```

This will create a `pkg/` folder containing the compiled `.wasm` binary and the JavaScript "glue" code.

## ⚛️ Integration with React (Vite)

1. **Install the local package:**
   In your React project root:
   ```bash
   npm install ./path/to/rust-wasm-lib/pkg
   ```

2. **Use it in your components:**

```javascript
import init, { greet } from 'rust-wasm-lib';

async function run() {
    // 1. Initialize the WASM module
    await init();

    // 2. Call Rust functions
    console.log(greet("Developer"));
}

run();
```

## 📝 Functions available
- `greet(name: string) -> string`: Returns a greeting from Rust.
- `expensive_calculation(a: number, b: number) -> number`: Performs math logic in Rust.


```
 Pro Tip: Automation
  If you find yourself making lots of changes, you can automate this so you don't have to keep switching terminals.

   1. Install cargo-watch:
cargo install cargo-watch

   2. Run a background watcher in the rust-wasm-lib folder:

cargo watch -i .gitignore -s "wasm-pack build --target web"
```
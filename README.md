# Rust Learning Project

Status: In Progress – Covering Core Concepts and Error Handling

This repository contains a structured, modular collection of Rust examples and exercises tailored for learners transitioning to Rust. The intent is to progressively build understanding and reference materials, promoting continuous evolution as you advance through your Rust learning journey. 

## Project Structure
The project is organized into a library with modules for each key concept.

```
├── src/
│   ├── borrowing.rs           # Examples of ownership and borrowing rules
│   ├── collections.rs         # Vector and HashMap examples
│   ├── enums.rs               # `enum` and `impl` usage
│   ├── error_handling.rs      # Demonstrating `Option` and `Result`
│   ├── file_io.rs             # File reading and command-line args
│   ├── iterators.rs           # Iterator patterns and `collect`
│   ├── lib.rs                 # Library root with `mod` declarations
│   ├── main.rs                # Main binary to run module examples
│   └── structs_and_data.rs    # Structs and data-enriched enums
└── Cargo.toml                 # Project manifest
```

## How to Run
Run all examples via the main binary:

```bash
cargo run
```

Run the file I/O example (make sure `test.txt` exists):

```bash
echo "Hello\nWorld\nFrom\nFile" > test.txt
cargo run -- test.txt
```

## Rust Learning Highlights
A concise snapshot of key topics covered so far:

### Why Rust?
Rust offers memory safety without garbage collection, high performance, and an expressive type system. It prioritizes immutability, explicit mutation, and strong error handling patterns with `Option` and `Result`.

### Core Language Features
- **Variables & Shadowing**: Immutable by default (`let`), mutable with `mut`. Shadowing allows reusing names.
- **Structs & Methods**: `struct` for data, `impl` for behavior—separating definitions promotes modularity over traditional classes.
- **Enums & Pattern Matching**: Rust `enum`s are powerful algebraic data types that can contain data and are used with `match` for expressive control flow.
- **Iterators & `collect`**: Iterators transform and process collections efficiently without unnecessary copying—e.g., `vec.iter().map(...).collect()`.
- **Memory & Ownership**: A clear memory model with the stack vs. heap, ownership rules, borrowing (`&`, `&mut`), and the borrow checker to ensure safety at compile time.
- **Error Handling**: Using `Option<T>` for values that can be absent and `Result<T, E>` for operations that can fail.

## To-Do List
- [x] Ownership and Borrowing Rules
- [x] Collections (Vector, HashMap)
- [x] Enums and Pattern Matching
- [x] Error Handling (`Option`, `Result`)
- [x] File I/O and Command-Line Arguments
- [x] Iterators and `collect`
- [x] Structs and Data-Enriched Enums
- [ ] Borrow Checker & Lifetime Rules (Deep Dive)
- [ ] Traits and Generics
- [ ] Advanced Traits & Generics
- [ ] Modules and Project Organization (Crates and Workspaces)
- [ ] Smart Pointers (`Box`, `Rc`, `RefCell`)
- [ ] Concurrency (Threads, Channels, `async/await`)
- [ ] Macros (Declarative and Procedural)
- [ ] Unsafe Rust and FFI (Foreign Function Interface)
- [ ] Testing (Unit, Integration, and Documentation Tests)
- [ ] Performance Optimization (Profiling, Benchmarking)


//! This module demonstrates the core ownership and borrowing rules in Rust.

// This function takes an IMMUTABLE reference (&) to a string.
// It can READ the data but cannot CHANGE it.
fn calculate_length(s: &String) -> usize {
    // s.push_str("!"); // This would cause a compiler error! The data is borrowed immutably.
    s.len()
}

// This function takes a MUTABLE reference (&mut) to a string.
// It can READ and CHANGE the data.
fn add_greeting(s: &mut String) {
    s.push_str(", world!");
}

pub fn run_examples() {
    println!("--- Demonstrating Ownership & Moving ---");
    // `s1` owns the String data allocated on the heap.
    let s1 = String::from("hello");

    // When we do this, the pointer to the data is copied, but ownership is MOVED to `s2`.
    // Rust invalidates `s1` to prevent a "double free" error where both variables might try to free the memory.
    let s2 = s1;
    println!("s2 has taken ownership of the value: '{}'", s2);
    // println!("Trying to use s1 after move: {}", s1); // This is a COMPILE ERROR. s1 is no longer valid.
    println!("`s1` is no longer valid after ownership was moved to `s2`.");

    println!("\n--- Demonstrating Immutable Borrowing (&) ---");
    // We can have MANY immutable references to the same data at once. It's safe because the data cannot be changed.
    //
    let message = String::from("This is a test.");
    let r1 = &message;
    let r2 = &message;
    println!(
        "Multiple immutable borrows are okay: r1='{}', r2='{}'",
        r1, r2
    );
    let len = calculate_length(&message);
    println!("The length of '{}' is {}.", message, len);

    println!("\n--- Demonstrating Mutable Borrowing (&mut) ---");
    // You can only have ONE mutable reference to a piece of data in a given scope.
    // This prevents data races at compile time.
    //
    let mut s_mut = String::from("hello");
    println!("Before mutation: '{}'", s_mut);

    let r_mut1 = &mut s_mut;
    // let r_mut2 = &mut s_mut; // COMPILE ERROR: cannot borrow `s_mut` as mutable more than once at a time.

    add_greeting(r_mut1);
    println!("After mutation: '{}'", s_mut);
}

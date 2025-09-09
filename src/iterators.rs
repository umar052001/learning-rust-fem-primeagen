//! This module demonstrates the power and flexibility of iterators in Rust.

pub fn run_examples() {
    // --- `map` and `collect` ---
    let original_vec = vec![1, 2, 3, 4];
    let new_vec: Vec<_> = original_vec.iter().map(|x| x + 1).collect();
    println!(
        "Original: {:?}, Mapped to new vec: {:?}",
        original_vec, new_vec
    );

    // --- `filter` and `count` ---
    let check_even_count = vec![1, 2, 3, 4, 5, 6]
        .iter()
        .filter(|&&x| x % 2 == 0)
        .count();
    println!("Count of even numbers: {}", check_even_count);

    // --- Chaining multiple iterators ---
    println!("Using skip, take_while, and for_each:");
    let data = vec![10, 20, 50, 40, 30];
    data.iter()
        .skip(1) // Skips the `10`
        .take_while(|&&x| x > 30) // Takes `20` (fails), `50` (takes), then stops at `40`
        .for_each(|x| println!("  - Found value: {}", x));

    // A better example for take_while
    println!("A better take_while example (take while less than 45):");
    let data2 = vec![10, 20, 50, 40, 30];
    data2
        .iter()
        .take_while(|&&x| x < 45)
        .for_each(|x| println!(" - {}", x));
}

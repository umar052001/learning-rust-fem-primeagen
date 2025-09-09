//! This module demonstrates common collections like Vec and HashMap.
use std::collections::HashMap;

/// Gets a number from a vector by index, multiplies it by 5, or returns a default.
/// Using `Option` is often safer than `unwrap_or`.
pub fn multiply(num_vec: &[usize], idx: usize) -> usize {
    // .get() returns an Option<&usize>
    // .copied() converts Option<&usize> to Option<usize>
    // .unwrap_or() provides a default value if the Option is None
    let value = num_vec.get(idx).copied().unwrap_or(0);
    value * 5
}

/// A function to run all the examples from this module.
pub fn run_examples() {
    // --- Vector Example ---
    let vector: Vec<usize> = vec![1, 2, 3];
    println!(
        "Multiplying index 2 of {:?}: {}",
        vector,
        multiply(&vector, 2)
    );
    println!(
        "Multiplying index 99 of {:?}: {}",
        vector,
        multiply(&vector, 99)
    );

    // --- HashMap Example ---
    // Create a HashMap from a vector of strings.
    let names = vec!["my", "name", "is", "umar"];
    let name_map: HashMap<&str, usize> = names
        .into_iter()
        .enumerate() // Creates pairs of (index, value)
        .map(|(idx, item)| (item, idx)) // Swaps them to (value, index)
        .collect(); // Collects into a HashMap

    println!("Created HashMap: {:?}", name_map);
}

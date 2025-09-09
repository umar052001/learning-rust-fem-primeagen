// Use the library crate we are building. The name is determined by your Cargo.toml,
// The `*` imports all the public modules we defined in `lib.rs`.
use learnrust::*;

fn main() {
    println!("--- Running Enum Examples ---");
    enums::run_examples();
    println!("\n");

    println!("--- Running Struct and Data Examples ---");
    structs_and_data::run_examples();
    println!("\n");

    println!("--- Running Collections Examples ---");
    collections::run_examples();
    println!("\n");

    println!("--- Running Iterator Examples ---");
    iterators::run_examples();
    println!("\n");

    println!("--- Running Borrowing Examples ---");
    borrowing::run_examples();
    println!("\n");

    println!("--- Running File I/O Example ---");
    // This function will handle its own output and errors.
    file_io::read_file_from_args();
    println!("\n");

    println!("--- All examples finished ---");
}

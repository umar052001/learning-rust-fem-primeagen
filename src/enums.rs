//! This module demonstrates the use of enums, impl blocks, and pattern matching.

// We can add attributes like `Debug` to allow the enum to be printed easily.
#[derive(Debug)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
}

impl Color {
    /// Checks if the color is Green using `if let`.
    pub fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        false
    }

    /// Checks if the color is a component of green (in additive color models) using `match`.
    pub fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Blue => true, // Blue is a primary component of Green on screens
            Color::Yellow => true, // Yellow is a primary component of Green in pigment
            Color::Green => true,
        }
    }
}

/// Prints a color name to the console based on the enum variant.
pub fn print_color(color: Color) {
    match color {
        Color::Red => println!("Color is: red"),
        Color::Yellow => println!("Color is: yellow"),
        Color::Green => println!("Color is: green"),
        Color::Blue => println!("Color is: blue"),
    }
}

/// A function to run all the examples from this module.
pub fn run_examples() {
    print_color(Color::Red);
    print_color(Color::Blue);

    let my_color = Color::Green;
    println!("Is the color green? {}", my_color.is_green());
    println!("Is the color part of green? {}", my_color.is_green_parts());
}

//! This module demonstrates structs and enums that hold data.

#[derive(Debug)]
pub struct Custom {
    pub name: String,
    pub age: usize,
}

#[derive(Debug)]
pub enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

/// Appends a new item to a vector of items.
pub fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello Umar!".to_string()));
}

/// A function to run all the examples from this module.
pub fn run_examples() {
    let mut items: Vec<Item> = Vec::new();
    println!("Items before append: {:?}", items);

    append(&mut items);
    items.push(Item::Number(42));
    items.push(Item::MyCustom(Custom {
        name: "Jani".to_string(),
        age: 30,
    }));

    println!("Items after append: {:?}", items);

    // This shows how to pattern match on enums with data
    for item in items {
        match item {
            Item::Number(n) => println!("Found a number: {}", n),
            Item::String(s) => println!("Found a string: '{}'", s),
            Item::MyCustom(c) => println!("Found a custom struct: name={}, age={}", c.name, c.age),
        }
    }
}

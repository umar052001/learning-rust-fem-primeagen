//! This module demonstrates the core borrowchecker and lifetime rules in Rust.
#[derive(Debug)]
struct Item {
    count: usize,
}
fn add_one(item: &mut Item) {
    item.count += 1;
}
fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item)
    }
}
pub fn run_examples() {
    println!("---This is an example of borrowchecker with mutable reference---");
    let mut item = Item { count: 1 };
    println!("{:?}", item);
    add_one(&mut item);
    println!("{:?}", item);
    println!(
        "---This is an example of borrowchecker with mutable reference breaking some rules---"
    );
    println!("--item is now a vector");
    let mut item = vec![Item { count: 1 }];
    let first = item.get_mut(0);
    println!("{:?}", first);
    let second = item.get_mut(1);
    // println!("{:?}", first); //uncomment this and comment above first and you will see an error
    println!("{:?}", second);
    println!("--printing items--");
    print_all(&item);
}

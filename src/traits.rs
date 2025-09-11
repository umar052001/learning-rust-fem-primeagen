//! This module demonstrates the core borrowchecker and lifetime rules in Rust.

use crate::shapes::{Area, Circle, Rect};
pub fn run_examples() {
    let rect = Rect::default();
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    println!("{}", circle.area());
    println!("{}", rect.area());
    for point in &rect {}
    println!("{}", rect)
}

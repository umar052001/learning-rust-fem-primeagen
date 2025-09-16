//! This module demonstrates the core borrowchecker and lifetime rules in Rust.

use crate::{
    collisions::Colidable,
    shapes::{Area, Circle, Rect},
};
pub fn run_examples() {
    let rect = Rect::default();
    let rect2 = Rect::default();
    // implement collide method for circle in shapes file to use circ.collide
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    println!("{}", circle.area());
    let circle2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 6.0,
    };
    circle.collide(&circle2);
    rect.collide(&rect2);
    rect.collide(&circle);
    println!("{}", rect.area());
    for point in &rect {}
    println!("{}", rect)
}

#![allow(dead_code)]

use rustcarte;

mod basic;
mod guess;
mod scoping;
mod stdlib;

fn main() {
    println!("rustcarte:{}", rustcarte::add(3, 4));
}

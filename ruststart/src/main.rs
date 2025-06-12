
#![allow(dead_code)]

use rustcarte;

mod basic;
mod stdlib;
mod scoping;
mod guess;

 fn main() {
    println!("rustcarte:{}", rustcarte::add(3, 4));
}
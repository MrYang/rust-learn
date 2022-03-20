
#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::{io, thread};
use std::time::Duration;

mod guess;
mod basic;

mod scoping;
mod stdlib;

use rustcarte;


fn main() {
    //guess::g();
    basic::pub_method();
    scoping::s();
    stdlib::s();

    println!("rustcarte:{}", rustcarte::add(3, 4));
}
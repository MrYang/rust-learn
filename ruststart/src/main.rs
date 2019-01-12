
#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::{io, thread};
use std::time::Duration;

mod guess;
mod types;
mod convert;
mod matching;
mod closures;

mod scoping;
mod stdlib;

use rustcarte;


fn main() {
    //guess::g();
    println!("const pi:{}", types::CONST_PI);
    types::t();
    convert::c();
    matching::m();
    scoping::s();
    stdlib::s();

    println!("rustcarte:{}", rustcarte::add(3, 4));

    closures::c();
}
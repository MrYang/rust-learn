extern crate rustcarte;

mod lifetimes;
mod closures;

pub fn s() {
    let a = rustcarte::add(3, 4);
    println!("add:{}", a);
}